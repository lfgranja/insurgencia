#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Cliente low-poly jogável: um avatar caminha entre quatro ilhas sem textura.
//!
//! Toda a geometria é procedural e de baixa poligonagem: prismas de poucos
//! lados representam as ilhas e um cone de seis lados representa o jogador.

use bevy::mesh::{ConeMeshBuilder, CylinderMeshBuilder};
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::window::PrimaryWindow;
use insurgencia_cliente::{
    deslocar, estado_inicial, ilha_em, EstadoJogador, Geometria3d, ILHAS, LIMITE_X, LIMITE_Y,
    VELOCIDADE,
};
use insurgencia_sim::Mundo;

/// Identificador do jogador local no espelho da simulação.
const PEER_ID: u64 = 1;

/// Espessura dos prismas que formam as ilhas, em unidades do mundo.
const ESPESSURA_ILHA: f32 = 18.0;

/// Altura do cone que representa o avatar, em unidades do mundo.
const ALTURA_AVATAR: f32 = 44.0;

/// Raio da base do cone que representa o avatar, em unidades do mundo.
const RAIO_AVATAR: f32 = 16.0;

/// Estado do jogador controlado nesta instância.
#[derive(Resource)]
struct JogadorLocal(EstadoJogador);

/// Espelho autoritativo mantido pela crate de simulação.
#[derive(Resource)]
struct Simulacao(Mundo);

/// Quando verdadeiro, a janela inicia minizada (opção `--minimizado`).
#[derive(Resource)]
struct ModoJanela(bool);

/// Marca a malha do avatar.
#[derive(Component)]
struct Avatar;

/// Marca o texto de estado exibido na tela.
#[derive(Component)]
struct TextoEstado;

fn main() {
    let minimizado = std::env::args().any(|arg| arg == "--minimizado");
    App::new()
        .insert_resource(ModoJanela(minimizado))
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

/// Monta a câmera 3D inclinada sobre o mapa, apontando para a origem.
#[must_use]
fn transformar_camera() -> Transform {
    let mut transform = Transform::from_xyz(0.0, 700.0, 500.0);
    transform.look_at(Vec3::ZERO, Dir3::NEG_Z);
    transform
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn configurar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materiais: ResMut<Assets<StandardMaterial>>,
    mut janelas: Query<&mut Window, With<PrimaryWindow>>,
    modo: Res<ModoJanela>,
) {
    if modo.0 {
        if let Some(mut janela) = janelas.iter_mut().next() {
            janela.set_minimized(true);
        }
    }

    commands.spawn((
        Camera3d::default(),
        transformar_camera(),
        AmbientLight {
            color: Color::srgb(0.72, 0.78, 0.92),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.97, 0.9),
            illuminance: 9_000.0,
            ..default()
        },
        Transform::from_xyz(-320.0, 760.0, 260.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));

    let jogador = estado_inicial();
    let (x, y) = jogador.posicao;

    let malha_avatar = meshes.add(ConeMeshBuilder::new(RAIO_AVATAR, ALTURA_AVATAR, 6));
    let material_avatar = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.98, 0.98, 1.0),
        ..default()
    });
    commands.spawn((
        Mesh3d(malha_avatar),
        MeshMaterial3d(material_avatar),
        Transform::from_xyz(x, ALTURA_AVATAR / 2.0, -y),
        Avatar,
    ));

    for ilha in &ILHAS {
        let (vermelho, verde, azul) = ilha.cor;
        let material = materiais.add(StandardMaterial {
            base_color: Color::srgb(vermelho, verde, azul),
            ..default()
        });
        let malha = match ilha.forma.geometria_3d() {
            Geometria3d::Caixa {
                largura,
                profundidade,
            } => meshes.add(Cuboid {
                half_size: Vec3::new(largura / 2.0, ESPESSURA_ILHA / 2.0, profundidade / 2.0),
            }),
            Geometria3d::Prisma { raio, lados } => {
                meshes.add(CylinderMeshBuilder::new(raio, ESPESSURA_ILHA, lados))
            }
        };
        commands.spawn((
            Mesh3d(malha),
            MeshMaterial3d(material),
            Transform::from_xyz(ilha.centro.0, ESPESSURA_ILHA / 2.0, -ilha.centro.1),
        ));
    }

    commands.spawn((
        Text::new("INSURGÊNCIA — low-poly"),
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

    let mut sim = Mundo::new();
    sim.adicionar_jogador(PEER_ID);
    commands.insert_resource(JogadorLocal(jogador));
    commands.insert_resource(Simulacao(sim));
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

/// Copia a posição do estado para a transformação 3D do avatar no plano do mapa.
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
        transform.translation.z = -jogador.0.posicao.1;
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
        "INSURGÊNCIA — low-poly\nPosicao: ({x:.0}, {y:.0})\nIlha: {ilha}\nLimites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}\nWASD ou setas para mover"
    );

    for mut texto in &mut textos {
        (**texto).clone_from(&conteudo);
    }
}
