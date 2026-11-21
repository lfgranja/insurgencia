#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Cliente greybox jogável: um avatar caminha entre quatro ilhas geométricas.

use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::window::PrimaryWindow;
use insurgencia_cliente::{
    deslocar, estado_inicial, ilha_em, EstadoJogador, ILHAS, LIMITE_X, LIMITE_Y, VELOCIDADE,
};
use insurgencia_sim::Mundo;

/// Identificador do jogador local no espelho da simulação.
const PEER_ID: u64 = 1;

/// Estado do jogador controlado nesta instância.
#[derive(Resource)]
struct JogadorLocal(EstadoJogador);

/// Espelho autoritativo mantido pela crate de simulação.
#[derive(Resource)]
struct Simulacao(Mundo);

/// Marca o sprite do avatar.
#[derive(Component)]
struct Avatar;

/// Marca o texto de estado exibido na tela.
#[derive(Component)]
struct TextoEstado;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, configurar)
        .add_systems(
            Update,
            (
                mover_jogador,
                sincronizar_avatar,
                sincronizar_sim,
                atualizar_texto,
            )
                .chain(),
        )
        .run();
}

/// Cria a câmera, as ilhas, o avatar e o painel de estado.
fn configurar(mut commands: Commands, mut janelas: Query<&mut Window, With<PrimaryWindow>>) {
    if let Some(mut janela) = janelas.iter_mut().next() {
        janela.set_minimized(true);
    }

    commands.spawn(Camera2d);

    let jogador = estado_inicial();
    let (x, y) = jogador.posicao;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.98, 0.98, 1.0),
            ..Sprite::sized(Vec2::splat(28.0))
        },
        Transform::from_xyz(x, y, 1.0),
        Avatar,
    ));

    for ilha in &ILHAS {
        let (largura, altura) = ilha.forma.tamanho();
        let (vermelho, verde, azul) = ilha.cor;
        commands.spawn((
            Sprite {
                color: Color::srgb(vermelho, verde, azul),
                ..Sprite::sized(Vec2::new(largura, altura))
            },
            Transform {
                translation: Vec3::new(ilha.centro.0, ilha.centro.1, 0.0),
                rotation: Quat::from_rotation_z(ilha.forma.rotacao()),
                ..default()
            },
        ));
    }

    commands.spawn((
        Text::new("INSURGÊNCIA — greybox"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.94, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(14.0),
            left: px(14.0),
            ..default()
        },
        TextoEstado,
    ));

    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(PEER_ID);
    commands.insert_resource(JogadorLocal(jogador));
    commands.insert_resource(Simulacao(mundo));
}

/// Lê `WASD` ou as setas e desloca o avatar com passo dependente do tempo.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn mover_jogador(
    teclado: Res<ButtonInput<KeyCode>>,
    tempo: Res<Time>,
    mut jogador: ResMut<JogadorLocal>,
) {
    let mut horizontal = 0.0_f32;
    let mut vertical = 0.0_f32;

    if teclado.pressed(KeyCode::KeyW) || teclado.pressed(KeyCode::ArrowUp) {
        vertical += 1.0;
    }
    if teclado.pressed(KeyCode::KeyS) || teclado.pressed(KeyCode::ArrowDown) {
        vertical -= 1.0;
    }
    if teclado.pressed(KeyCode::KeyA) || teclado.pressed(KeyCode::ArrowLeft) {
        horizontal -= 1.0;
    }
    if teclado.pressed(KeyCode::KeyD) || teclado.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }

    let direcao = Vec2::new(horizontal, vertical);
    if direcao.length_squared() <= f32::EPSILON {
        return;
    }

    let passo = direcao.normalize() * VELOCIDADE * tempo.delta_secs();
    jogador.0 = deslocar(jogador.0, (passo.x, passo.y));
}

/// Copia a posição do estado para a transformação do sprite do avatar.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn sincronizar_avatar(
    jogador: Res<JogadorLocal>,
    mut avatares: Query<&mut Transform, With<Avatar>>,
) {
    for mut transform in &mut avatares {
        transform.translation.x = jogador.0.posicao.0;
        transform.translation.y = jogador.0.posicao.1;
    }
}

/// Mantém o espelho da simulação alinhado ao estado local.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn sincronizar_sim(jogador: Res<JogadorLocal>, mut simulacao: ResMut<Simulacao>) {
    let Some(remoto) = simulacao.0.jogadores.get_mut(&PEER_ID) else {
        return;
    };
    let (x, y) = jogador.0.posicao;
    remoto.posicao = (f64::from(x), f64::from(y));
}

/// Atualiza o painel com posição, ilha corrente e limites do mapa.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn atualizar_texto(jogador: Res<JogadorLocal>, mut textos: Query<&mut Text, With<TextoEstado>>) {
    let (x, y) = jogador.0.posicao;
    let ilha = if let Some(atual) = ilha_em((x, y)) {
        atual.nome
    } else {
        "nenhuma"
    };
    let conteudo = format!(
        "INSURGÊNCIA — greybox\nPosicao: ({x:.0}, {y:.0})\nIlha: {ilha}\nLimites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}\nWASD ou setas para mover"
    );

    for mut texto in &mut textos {
        (**texto).clone_from(&conteudo);
    }
}
