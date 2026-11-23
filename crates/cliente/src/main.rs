#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Cliente low-poly jogável: terreno irregular, ilhas em talude e avatar facetado.
//!
//! Toda a geometria é procedural e gerada em tempo de execução, sem texturas; a
//! fonte TrueType da HUD é o único asset externo. As malhas não usam índices
//! compartilhados, de modo que cada triângulo recebe sua própria normal e as
//! facetas ficam visíveis.

use std::f32::consts::TAU;
use std::path::Path;

use bevy::asset::{AssetPlugin, RenderAssetUsages};
use bevy::mesh::{
    ConeMeshBuilder, CylinderMeshBuilder, Mesh, PrimitiveTopology, SphereKind, SphereMeshBuilder,
};
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::window::PrimaryWindow;
use insurgencia_cliente::{
    deslocar, estado_inicial, ilha_em, EstadoJogador, Geometria3d, Ilha, ILHAS, LIMITE_X, LIMITE_Y,
    VELOCIDADE,
};
use insurgencia_sim::{
    Comando, Mundo, ResultadoPartida, MAX_RECONQUISTAS, NIVEL_MAXIMO, PASSO_SEGUNDOS, TICKS_COLETA,
    TICKS_CONSTRUCAO_BARREIRA,
};

/// Identificador do jogador local no espelho da simulação.
const PEER_ID: u64 = 1;

/// Identificador do alvo de treino no espelho da simulação (scaffolding não-canônico).
const PEER_ALVO_TREINO: u64 = 2;

/// Posição fixa do alvo de treino no plano do mapa (scaffolding não-canônico).
const ALVO_TREINO_POSICAO: (f64, f64) = (2.0, 0.0);

/// Quantidade de vértices no contorno irregular de cada ilha.
const LADOS_ILHA: u16 = 8;

/// Divisões do terreno nos eixos X e Z.
const COLUNAS_TERRENO: u16 = 16;
const LINHAS_TERRENO: u16 = 10;

/// Altura média do topo das ilhas, onde o avatar apoia os pés.
const ALTURA_ILHA: f32 = 18.0;

/// Raio da base do avatar, em unidades do mundo.
const RAIO_AVATAR: f32 = 16.0;

/// Tons de verde-oliva usados pelas manchas do terreno.
const CORES_TERRENO: [[f32; 3]; 3] = [[0.29, 0.33, 0.18], [0.36, 0.39, 0.22], [0.24, 0.29, 0.16]];

/// Estado do jogador controlado nesta instância.
#[derive(Resource)]
struct JogadorLocal(EstadoJogador);

/// Espelho autoritativo mantido pela crate de simulação.
#[derive(Resource)]
struct Simulacao(Mundo);

/// Sequências locais usadas nos comandos enviados à simulação.
#[derive(Resource)]
struct ComandoLocal {
    seq: i64,
    request_id: i64,
}

/// Tempo acumulado para avançar a simulação em passos fixos.
#[derive(Resource)]
struct AccumuladorTick(f64);

/// Quando verdadeiro, a janela inicia minizada (opção `--minimizado`).
#[derive(Resource)]
struct ModoJanela(bool);

/// Marca a entidade raiz do avatar.
#[derive(Component)]
struct Avatar;

/// Marca o marcador estático do alvo de treino.
#[derive(Component)]
struct AlvoTreino;

/// Marca o texto de estado exibido na tela.
#[derive(Component)]
struct TextoEstado;

fn main() {
    let minimizado = std::env::args().any(|arg| arg == "--minimizado");
    App::new()
        .insert_resource(ModoJanela(minimizado))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: caminho_assets(),
            ..default()
        }))
        .add_systems(Startup, configurar)
        .add_systems(
            Update,
            (
                mover_jogador,
                sincronizar_avatar,
                solicitar_barreira,
                solicitar_coleta,
                solicitar_ponte,
                solicitar_propaganda,
                solicitar_ataque,
                solicitar_propaganda_no,
                solicitar_atacar_no,
                sincronizar_sim,
                avancar_simulacao,
                atualizar_texto,
            )
                .chain(),
        )
        .run();
}

/// Caminho absoluto da pasta `assets`, sem depender de `BEVY_ASSET_ROOT`.
///
/// O Bevy 0.20 usa o diretório do executável como base quando não há variável
/// de ambiente, o que aponta para `target/debug` e não encontra a fonte da HUD.
/// Procuramos a pasta ao lado do executável, na raiz do workspace
/// (`target/debug/../..`) e, por fim, no caminho derivado do manifesto da crate
/// cliente em tempo de compilação.
#[must_use]
fn caminho_assets() -> String {
    let raiz_exe = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf));
    let raiz_manifesto = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent);

    [
        raiz_exe.as_deref().map(|raiz| raiz.join("assets")),
        raiz_exe.as_deref().map(|raiz| raiz.join("../../assets")),
        raiz_manifesto.map(|raiz| raiz.join("assets")),
    ]
    .into_iter()
    .flatten()
    .find_map(|caminho| caminho.canonicalize().ok().filter(|real| real.is_dir()))
    .map_or_else(
        || format!("{}/../../assets", env!("CARGO_MANIFEST_DIR")),
        |caminho| caminho.to_string_lossy().into_owned(),
    )
}

/// Monta a câmera 3D inclinada sobre o mapa, apontando para a origem.
#[must_use]
fn transformar_camera() -> Transform {
    let mut transform = Transform::from_xyz(0.0, 760.0, 540.0);
    transform.look_at(Vec3::ZERO, Dir3::NEG_Z);
    transform
}

/// Número determinístico entre zero e um, derivado de índices inteiros.
#[must_use]
fn fator(x: u16, z: u16, sal: u16) -> f32 {
    let valor = f32::from(x) * 12.9898 + f32::from(z) * 78.233 + f32::from(sal) * 37.719;
    (valor.sin() * 43_758.547).fract().abs()
}

/// Malha não indexada: `compute_normals` gera uma normal por triângulo.
#[must_use]
fn malha(posicoes: Vec<[f32; 3]>) -> Mesh {
    let mut resultado = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    resultado.insert_attribute(Mesh::ATTRIBUTE_POSITION, posicoes);
    resultado.compute_normals();
    resultado
}

/// Converte uma malha de primitiva em facetas com normais próprias.
#[must_use]
fn achatar(builder: impl Into<Mesh>) -> Mesh {
    let mut resultado: Mesh = builder.into();
    resultado.duplicate_vertices();
    resultado.compute_normals();
    resultado
}

/// Altura de um vértice do terreno, variando de forma determinística.
#[must_use]
fn altura_terreno(coluna: u16, linha: u16) -> f32 {
    -28.0 + fator(coluna, linha, 3) * 12.0
}

/// Posição de um vértice da malha do terreno no espaço do mundo.
#[must_use]
fn ponto_terreno(coluna: u16, linha: u16) -> [f32; 3] {
    let x = -LIMITE_X + 2.0 * LIMITE_X * f32::from(coluna) / f32::from(COLUNAS_TERRENO);
    let z = -LIMITE_Y + 2.0 * LIMITE_Y * f32::from(linha) / f32::from(LINHAS_TERRENO);
    [x, altura_terreno(coluna, linha), z]
}

/// Distância do centro de uma ilha até a borda da sua forma, num dado ângulo.
#[must_use]
fn borda_ilha(geometria: Geometria3d, angulo: f32) -> f32 {
    let cosseno = angulo.cos().abs().max(0.001);
    let seno = angulo.sin().abs().max(0.001);
    match geometria {
        Geometria3d::Caixa {
            largura,
            profundidade,
        } => (largura / 2.0 / cosseno).min(profundidade / 2.0 / seno),
        Geometria3d::Prisma { raio, .. } => raio / (cosseno + seno),
    }
}

/// Cria o terreno irregular em três manchas de verde-oliva.
fn criar_terreno(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    materiais: &mut Assets<StandardMaterial>,
) {
    let mut partes: [Vec<[f32; 3]>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for coluna in 0..COLUNAS_TERRENO {
        for linha in 0..LINHAS_TERRENO {
            let p00 = ponto_terreno(coluna, linha);
            let p01 = ponto_terreno(coluna, linha + 1);
            let p10 = ponto_terreno(coluna + 1, linha);
            let p11 = ponto_terreno(coluna + 1, linha + 1);
            let tom = fator(coluna, linha, 7);
            let indice = if tom < 0.34 {
                0
            } else if tom < 0.67 {
                1
            } else {
                2
            };
            let alvo = &mut partes[indice];
            if fator(coluna, linha, 11) < 0.5 {
                alvo.extend_from_slice(&[p00, p01, p10, p10, p01, p11]);
            } else {
                alvo.extend_from_slice(&[p00, p01, p11, p00, p11, p10]);
            }
        }
    }

    for (posicoes, cor) in partes.iter().zip(CORES_TERRENO.iter()) {
        let [vermelho, verde, azul] = *cor;
        let material = materiais.add(StandardMaterial {
            base_color: Color::srgb(vermelho, verde, azul),
            perceptual_roughness: 1.0,
            ..default()
        });
        comandos.spawn((
            Mesh3d(malhas.add(malha(posicoes.clone()))),
            MeshMaterial3d(material),
            Transform::IDENTITY,
        ));
    }
}

/// Cria uma árvore de tronco pentagonal e copa hexagonal.
fn criar_arvore(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    tronco: Handle<StandardMaterial>,
    copa: Handle<StandardMaterial>,
    ponto: [f32; 3],
) {
    let madeira = achatar(CylinderMeshBuilder::new(3.2, 14.0, 5));
    let folhagem = achatar(ConeMeshBuilder::new(11.0, 26.0, 6));
    comandos.spawn((
        Mesh3d(malhas.add(madeira)),
        MeshMaterial3d(tronco),
        Transform::from_xyz(ponto[0], ponto[1] + 7.0, ponto[2]),
    ));
    comandos.spawn((
        Mesh3d(malhas.add(folhagem)),
        MeshMaterial3d(copa),
        Transform::from_xyz(ponto[0], ponto[1] + 27.0, ponto[2]),
    ));
}

/// Cria uma rocha baixa de cinco lados.
fn criar_rocha(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    ponto: [f32; 3],
    giracao: f32,
) {
    let pedra = achatar(ConeMeshBuilder::new(8.0, 7.0, 5));
    comandos.spawn((
        Mesh3d(malhas.add(pedra)),
        MeshMaterial3d(material),
        Transform::from_xyz(ponto[0], ponto[1] + 3.5, ponto[2])
            .with_rotation(Quat::from_rotation_y(giracao)),
    ));
}

/// Distribui árvores e rochas sobre o topo da ilha a partir dos vértices do contorno.
fn decorar_ilha(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    materiais: &mut Assets<StandardMaterial>,
    indice: u16,
    contorno: &[[f32; 3]],
    centro_topo: [f32; 3],
) {
    let tronco = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.36, 0.23, 0.13),
        perceptual_roughness: 1.0,
        ..default()
    });
    let copa = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.11, 0.32, 0.16),
        perceptual_roughness: 0.9,
        ..default()
    });
    let pedra = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.44, 0.46, 0.47),
        perceptual_roughness: 1.0,
        ..default()
    });

    for ordem in [0_u16, 1, 2] {
        let vertice = match ordem {
            0 => 1_u16,
            1 => 3_u16,
            _ => 6_u16,
        };
        let alvo = contorno[usize::from(vertice)];
        let proximidade = 0.45 + 0.3 * fator(ordem, indice, 23);
        let ponto = [
            centro_topo[0] + (alvo[0] - centro_topo[0]) * proximidade,
            centro_topo[1] + (alvo[1] - centro_topo[1]) * proximidade,
            centro_topo[2] + (alvo[2] - centro_topo[2]) * proximidade,
        ];
        criar_arvore(comandos, malhas, tronco.clone(), copa.clone(), ponto);
    }

    for ordem in [0_u16, 1] {
        let vertice = if ordem == 0 { 0_u16 } else { 5_u16 };
        let alvo = contorno[usize::from(vertice)];
        let proximidade = 0.5 + 0.25 * fator(ordem, indice, 29);
        let ponto = [
            centro_topo[0] + (alvo[0] - centro_topo[0]) * proximidade,
            centro_topo[1] + (alvo[1] - centro_topo[1]) * proximidade,
            centro_topo[2] + (alvo[2] - centro_topo[2]) * proximidade,
        ];
        let giracao = TAU * fator(ordem, indice, 31);
        criar_rocha(comandos, malhas, pedra.clone(), ponto, giracao);
    }
}

/// Cria uma ilha irregular com topo triangulado, bordas em talude e vegetação.
fn criar_ilha(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    materiais: &mut Assets<StandardMaterial>,
    ilha: &Ilha,
    indice: u16,
) {
    let geometria = ilha.forma.geometria_3d();
    let (centro_x, centro_y) = ilha.centro;
    let mut contorno_base: Vec<[f32; 3]> = Vec::new();
    let mut contorno_topo: Vec<[f32; 3]> = Vec::new();

    for lado in 0..LADOS_ILHA {
        let angulo = TAU * f32::from(lado) / f32::from(LADOS_ILHA);
        let perturbacao = 0.88 + 0.24 * fator(lado, indice, 5);
        let raio = borda_ilha(geometria, angulo) * perturbacao;
        let deslocamento_x = angulo.cos() * raio;
        let deslocamento_z = -angulo.sin() * raio;
        contorno_base.push([
            centro_x + deslocamento_x,
            -14.0 + fator(lado, indice, 13) * 6.0,
            -centro_y + deslocamento_z,
        ]);
        contorno_topo.push([
            centro_x + deslocamento_x * 0.76,
            ALTURA_ILHA - 3.0 + fator(lado, indice, 17) * 7.0,
            -centro_y + deslocamento_z * 0.76,
        ]);
    }

    let centro_topo = [centro_x, ALTURA_ILHA + 3.0, -centro_y];
    let centro_base = [centro_x, -20.0, -centro_y];
    let mut topo = Vec::new();
    let mut talude = Vec::new();
    for atual in 0..usize::from(LADOS_ILHA) {
        let proximo = (atual + 1) % contorno_topo.len();
        let t0 = contorno_topo[atual];
        let t1 = contorno_topo[proximo];
        let b0 = contorno_base[atual];
        let b1 = contorno_base[proximo];
        topo.extend_from_slice(&[centro_topo, t0, t1]);
        talude.extend_from_slice(&[b0, b1, t1, b0, t1, t0, centro_base, b1, b0]);
    }

    let (vermelho, verde, azul) = ilha.cor;
    let material_topo = materiais.add(StandardMaterial {
        base_color: Color::srgb(vermelho, verde, azul),
        perceptual_roughness: 0.85,
        ..default()
    });
    let material_talude = materiais.add(StandardMaterial {
        base_color: Color::srgb(vermelho * 0.55, verde * 0.55, azul * 0.55),
        perceptual_roughness: 1.0,
        ..default()
    });
    comandos.spawn((
        Mesh3d(malhas.add(malha(topo))),
        MeshMaterial3d(material_topo),
        Transform::IDENTITY,
    ));
    comandos.spawn((
        Mesh3d(malhas.add(malha(talude))),
        MeshMaterial3d(material_talude),
        Transform::IDENTITY,
    ));
    decorar_ilha(
        comandos,
        malhas,
        materiais,
        indice,
        &contorno_topo,
        centro_topo,
    );
}

/// Cria o avatar com base, corpo cônico e cabeça esférica facetada.
fn criar_avatar(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    materiais: &mut Assets<StandardMaterial>,
    posicao: (f32, f32),
) {
    let armadura = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.94, 0.95, 1.0),
        perceptual_roughness: 0.7,
        ..default()
    });
    let cabeca = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.96, 0.76, 0.55),
        perceptual_roughness: 0.9,
        ..default()
    });
    let base = achatar(CylinderMeshBuilder::new(RAIO_AVATAR, 8.0, 6));
    let corpo = achatar(ConeMeshBuilder::new(RAIO_AVATAR * 0.82, 30.0, 6));
    let cranio = achatar(SphereMeshBuilder::new(
        9.0,
        SphereKind::Ico { subdivisions: 1 },
    ));
    let material_corpo = armadura.clone();

    comandos
        .spawn((
            Transform::from_xyz(posicao.0, ALTURA_ILHA + 3.0, -posicao.1),
            Avatar,
            Visibility::default(),
        ))
        .with_children(|filho| {
            filho.spawn((
                Mesh3d(malhas.add(base)),
                MeshMaterial3d(material_corpo.clone()),
                Transform::from_xyz(0.0, 4.0, 0.0),
            ));
            filho.spawn((
                Mesh3d(malhas.add(corpo)),
                MeshMaterial3d(material_corpo),
                Transform::from_xyz(0.0, 23.0, 0.0),
            ));
            filho.spawn((
                Mesh3d(malhas.add(cranio)),
                MeshMaterial3d(cabeca),
                Transform::from_xyz(0.0, 47.0, 0.0),
            ));
        });
}

/// Cria o marcador cônico do alvo de treino, estático sobre a ilha.
#[expect(
    clippy::cast_possible_truncation,
    reason = "As coordenadas do alvo sao literais exatos e cabem em f32"
)]
fn criar_alvo_treino(
    comandos: &mut Commands,
    malhas: &mut Assets<Mesh>,
    materiais: &mut Assets<StandardMaterial>,
) {
    let material = materiais.add(StandardMaterial {
        base_color: Color::srgb(0.86, 0.24, 0.18),
        perceptual_roughness: 0.8,
        ..default()
    });
    let malha = achatar(ConeMeshBuilder::new(RAIO_AVATAR * 0.6, 18.0, 4));
    let (x, y) = ALVO_TREINO_POSICAO;

    comandos
        .spawn((
            Transform::from_xyz(x as f32, ALTURA_ILHA + 3.0, -(y as f32)),
            AlvoTreino,
            Visibility::default(),
        ))
        .with_children(|filho| {
            filho.spawn((
                Mesh3d(malhas.add(malha)),
                MeshMaterial3d(material),
                Transform::from_xyz(0.0, 9.0, 0.0),
            ));
        });
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
    fontes: Res<AssetServer>,
) {
    if modo.0 {
        if let Some(mut janela) = janelas.iter_mut().next() {
            janela.set_minimized(true);
        }
    }

    let fonte: Handle<Font> = fontes.load("fonts/NotoSans-Regular.ttf");

    commands.spawn((
        Camera3d::default(),
        transformar_camera(),
        AmbientLight {
            color: Color::srgb(0.68, 0.74, 0.86),
            brightness: 55.0,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.96, 0.86),
            illuminance: 9_500.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-360.0, 820.0, 280.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));

    let jogador = estado_inicial();
    criar_terreno(&mut commands, &mut meshes, &mut materiais);
    for (indice, ilha) in ILHAS.iter().enumerate() {
        let indice_u16 = u16::try_from(indice).unwrap_or(0);
        criar_ilha(&mut commands, &mut meshes, &mut materiais, ilha, indice_u16);
    }
    criar_avatar(&mut commands, &mut meshes, &mut materiais, jogador.posicao);
    criar_alvo_treino(&mut commands, &mut meshes, &mut materiais);

    commands.spawn((
        Text::new("INSURGÊNCIA — low-poly"),
        TextFont {
            font: fonte.into(),
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
    sim.adicionar_jogador(PEER_ALVO_TREINO);
    if let Some(alvo) = sim.jogadores.get_mut(&PEER_ALVO_TREINO) {
        alvo.posicao = ALVO_TREINO_POSICAO;
    }
    commands.insert_resource(JogadorLocal(jogador));
    commands.insert_resource(ComandoLocal {
        seq: 0,
        request_id: 0,
    });
    commands.insert_resource(Simulacao(sim));
    commands.insert_resource(AccumuladorTick(0.0));
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

/// Agenda a construção de uma barreira no ponto atual ao pressionar `B`.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_barreira(
    teclado: Res<ButtonInput<KeyCode>>,
    jogador: Res<JogadorLocal>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyB) {
        return;
    }

    local.seq += 1;
    local.request_id += 1;
    let (x, y) = jogador.0.posicao;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "construir_barreira".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some((f64::from(x), f64::from(y))),
    });
}

/// Agenda a coleta de materiais no ponto atual ao pressionar `C`.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_coleta(
    teclado: Res<ButtonInput<KeyCode>>,
    jogador: Res<JogadorLocal>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyC) {
        return;
    }

    local.seq += 1;
    local.request_id += 1;
    let (x, y) = jogador.0.posicao;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "coletar_materiais".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some((f64::from(x), f64::from(y))),
    });
}

/// Agenda a construção de uma ponte provisória no ponto atual ao pressionar `P`.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_ponte(
    teclado: Res<ButtonInput<KeyCode>>,
    jogador: Res<JogadorLocal>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyP) {
        return;
    }

    local.seq += 1;
    local.request_id += 1;
    let (x, y) = jogador.0.posicao;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "construir_ponte".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some((f64::from(x), f64::from(y))),
    });
}

/// Agenda a propaganda no súdito no ponto atual ao pressionar `G`.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_propaganda(
    teclado: Res<ButtonInput<KeyCode>>,
    jogador: Res<JogadorLocal>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyG) {
        return;
    }

    local.seq += 1;
    local.request_id += 1;
    let (x, y) = jogador.0.posicao;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "propaganda".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some((f64::from(x), f64::from(y))),
    });
}

/// Agenda um ataque forte ao alvo de treino ao pressionar `Q`.
///
/// O alvo é scaffolding não-canônico em `ALVO_TREINO_POSICAO`; o núcleo recusa o
/// ataque quando o jogador está fora do alcance de contato de 2 m.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_ataque(
    teclado: Res<ButtonInput<KeyCode>>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyQ) {
        return;
    }

    local.seq += 1;
    local.request_id += 1;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "atacar_forte".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some(ALVO_TREINO_POSICAO),
    });
}

/// Devolve a posição do Nó mais próximo do jogador, com desempate pelo menor `id`.
///
/// A distância euclidiana ao quadrado preserva a ordem da distância e `total_cmp`
/// mantém a comparação determinística sem raiz quadrada. As posições dos Nós são
/// simplificação de protótipo `[HIPÓTESE]` da simulação.
#[must_use]
fn posicao_no_mais_proximo(mundo: &Mundo, jogador: (f32, f32)) -> Option<(f64, f64)> {
    let (x, y) = (f64::from(jogador.0), f64::from(jogador.1));
    mundo
        .nos
        .values()
        .min_by(|a, b| {
            let da = (a.posicao.0 - x).mul_add(a.posicao.0 - x, (a.posicao.1 - y).powi(2));
            let db = (b.posicao.0 - x).mul_add(b.posicao.0 - x, (b.posicao.1 - y).powi(2));
            da.total_cmp(&db).then_with(|| a.id.cmp(&b.id))
        })
        .map(|no| no.posicao)
}

/// Agenda Propaganda no Nó mais próximo ao pressionar `R`.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_propaganda_no(
    teclado: Res<ButtonInput<KeyCode>>,
    jogador: Res<JogadorLocal>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyR) {
        return;
    }
    let Some(destino) = posicao_no_mais_proximo(&simulacao.0, jogador.0.posicao) else {
        return;
    };
    local.seq += 1;
    local.request_id += 1;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "propaganda_no".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some(destino),
    });
}

/// Agenda dano no Nó mais próximo ao pressionar `F`; o dano nunca converte o Nó.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn solicitar_atacar_no(
    teclado: Res<ButtonInput<KeyCode>>,
    jogador: Res<JogadorLocal>,
    mut local: ResMut<ComandoLocal>,
    mut simulacao: ResMut<Simulacao>,
) {
    if !teclado.just_pressed(KeyCode::KeyF) {
        return;
    }
    let Some(destino) = posicao_no_mais_proximo(&simulacao.0, jogador.0.posicao) else {
        return;
    };
    local.seq += 1;
    local.request_id += 1;
    simulacao.0.agendar_comando(Comando {
        peer_id: PEER_ID,
        acao: "atacar_no".to_owned(),
        seq: local.seq,
        request_id: local.request_id,
        destino: Some(destino),
    });
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

/// Avança a simulação em passos fixos, com teto de quatro ticks por frame.
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn avancar_simulacao(
    tempo: Res<Time>,
    mut simulacao: ResMut<Simulacao>,
    mut acumulador: ResMut<AccumuladorTick>,
) {
    acumulador.0 += f64::from(tempo.delta_secs());
    for _ in 0..4 {
        if acumulador.0 < PASSO_SEGUNDOS {
            break;
        }
        simulacao.0.avancar_tick();
        acumulador.0 -= PASSO_SEGUNDOS;
    }
}

/// Descreve o desfecho da partida para o painel da HUD.
#[must_use]
fn texto_resultado(simulacao: &Simulacao) -> String {
    match simulacao.0.resultado {
        Some(ResultadoPartida::Vitoria) => "Autogestão estabelecida".to_owned(),
        Some(ResultadoPartida::DerrotaPorTempo) => "Comuna dissolvida — tempo esgotado".to_owned(),
        Some(ResultadoPartida::DerrotaPorReconquista) => {
            format!("Comuna dissolvida — {MAX_RECONQUISTAS} reconquistas")
        }
        None => "em curso".to_owned(),
    }
}

/// Monta o texto do painel a partir do estado local e do espelho da simulação.
#[must_use]
fn texto_hud(jogador: &JogadorLocal, simulacao: &Simulacao) -> String {
    let (x, y) = jogador.0.posicao;
    let ilha = if let Some(atual) = ilha_em((x, y)) {
        atual.nome
    } else {
        "nenhuma"
    };
    let remoto = simulacao.0.jogadores.get(&PEER_ID);
    let materiais = remoto.map_or(0, |j| j.materiais);
    let suprimentos = remoto.map_or(0, |j| j.suprimentos);
    let autogestao = remoto.map_or(0, |j| j.pontos_autogestao);
    let folego = remoto.map_or(0, |j| j.folego);
    let vida = remoto.map_or(0, |j| j.vida);
    let vida_alvo = if let Some(alvo) = simulacao.0.jogadores.get(&PEER_ALVO_TREINO) {
        alvo.vida
    } else {
        0
    };
    let funcao = remoto.map_or("Artesão", |j| j.funcao.nome());
    let nivel = remoto.map_or(1, |j| j.nivel);
    let barreiras = simulacao
        .0
        .barreiras
        .values()
        .filter(|barreira| barreira.dono == PEER_ID)
        .count();
    let pontes = simulacao
        .0
        .pontes
        .values()
        .filter(|ponte| ponte.dono == PEER_ID)
        .count();
    let suditos = simulacao
        .0
        .suditos
        .values()
        .filter(|sudito| sudito.convertido)
        .count();
    let ilhas_controladas = simulacao
        .0
        .ilhas
        .values()
        .filter(|ilha| ilha.controlada)
        .count();
    let sinais = simulacao.0.sinais.len();
    let nos_convertidos = simulacao.0.nos.values().filter(|no| no.convertido).count();
    let resistencia = simulacao.0.resistencia_ideologica;
    let resultado = texto_resultado(simulacao);
    let construindo = match remoto {
        Some(j) if j.construcao_ponto.is_some() => {
            format!("{}/{}", j.construcao_ticks, TICKS_CONSTRUCAO_BARREIRA)
        }
        _ => "nao".to_owned(),
    };
    let coletando = match remoto {
        Some(j) if j.coleta_ponto.is_some() => {
            format!("{}/{}", j.coleta_ticks, TICKS_COLETA)
        }
        _ => "nao".to_owned(),
    };
    format!(
        concat!(
            "INSURGÊNCIA — low-poly\nPosicao: ({x:.0}, {y:.0})\nIlha: {ilha}\n",
            "Função: {funcao}\nNível: {nivel}/{NIVEL_MAXIMO}\n",
            "Limites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}\n",
            "Suprimentos: {suprimentos}\nMateriais: {materiais}\n",
            "Fôlego: {folego}\nVida: {vida}\nAlvo: {vida_alvo}\n",
            "Autogestão: {autogestao}\nBarreiras: {barreiras}\n",
            "Pontes: {pontes}\nSúditos: {suditos}\n",
            "Resistência: {resistencia}\nNós: {nos_convertidos}/3\n",
            "Ilhas: {ilhas_controladas}\nSinais: {sinais}\nResultado: {resultado}\n",
            "Construindo: {construindo}\nColeta: {coletando}\n",
            "WASD ou setas para mover · B para construir barreira · P para construir ponte · ",
            "C para coletar materiais · G para propaganda · R para propaganda no Nó · ",
            "F para atacar o Nó · Aproxime-se do alvo e use Q para atacar"
        ),
        x = x,
        y = y,
        ilha = ilha,
        funcao = funcao,
        nivel = nivel,
        NIVEL_MAXIMO = NIVEL_MAXIMO,
        LIMITE_X = LIMITE_X,
        LIMITE_Y = LIMITE_Y,
        suprimentos = suprimentos,
        materiais = materiais,
        folego = folego,
        vida = vida,
        vida_alvo = vida_alvo,
        autogestao = autogestao,
        barreiras = barreiras,
        pontes = pontes,
        suditos = suditos,
        resistencia = resistencia,
        nos_convertidos = nos_convertidos,
        ilhas_controladas = ilhas_controladas,
        sinais = sinais,
        resultado = resultado,
        construindo = construindo,
        coletando = coletando,
    )
}

/// Atualiza o painel com o texto montado por [`texto_hud`].
#[expect(
    clippy::needless_pass_by_value,
    reason = "Bevy 0.20 SystemParam exige Res<T> por valor; &Res<T> nao implementa SystemParam"
)]
fn atualizar_texto(
    jogador: Res<JogadorLocal>,
    simulacao: Res<Simulacao>,
    mut textos: Query<&mut Text, With<TextoEstado>>,
) {
    let conteudo = texto_hud(&jogador, &simulacao);
    for mut texto in &mut textos {
        (**texto).clone_from(&conteudo);
    }
}
