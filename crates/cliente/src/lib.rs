#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Layout puro e estado do jogador do cliente greybox do Núcleo Insurgência.
//!
//! Esta crate não depende da Bevy: o binário apenas consome as descrições
//! geométricas e o estado devolvidos aqui para desenhar a cena.

use std::f32::consts::{FRAC_PI_4, SQRT_2};

/// Meia-largura da área jogável, em unidades de tela.
pub const LIMITE_X: f32 = 580.0;

/// Meia-altura da área jogável, em unidades de tela.
pub const LIMITE_Y: f32 = 310.0;

/// Posição inicial do avatar no plano `(x, y)`.
pub const POSICAO_INICIAL: (f32, f32) = (0.0, -236.0);

/// Deslocamento do avatar por segundo, em unidades de tela.
pub const VELOCIDADE: f32 = 260.0;

/// Fator de conversão entre pixels de tela e metros da simulação.
///
/// Constante `[HIPÓTESE]` do protótipo: um metro equivale a 100 pixels, de modo
/// que os 260 px/s do cliente correspondem a 2,6 m/s na simulação.
pub const PIXELS_POR_METRO: f64 = 100.0;

/// Silhueta 3D de uma ilha, derivada da forma 2D sem qualquer textura.
#[derive(Clone, Copy, Debug)]
pub enum Geometria3d {
    /// Prisma retangular, com extensao total nos eixos X e Z.
    Caixa {
        /// Extensao total no eixo X.
        largura: f32,
        /// Extensao total no eixo Z.
        profundidade: f32,
    },
    /// Prisma de poucos lados, com vertices a `raio` do centro.
    Prisma {
        /// Distancia do centro a cada vertice da base.
        raio: f32,
        /// Quantidade de lados da base (baixa poligonagem).
        lados: u32,
    },
}

/// Geometria usada para desenhar e testar uma ilha.
#[derive(Clone, Copy, Debug)]
pub enum Forma {
    /// Retângulo alinhado aos eixos.
    Retangulo {
        /// Largura total.
        largura: f32,
        /// Altura total.
        altura: f32,
    },
    /// Quadrado girado em 45 graus, com cada vértice a `raio` do centro.
    Losango {
        /// Distância do centro a cada vértice.
        raio: f32,
    },
}

impl Forma {
    /// Dimensões do sprite que representa a forma.
    #[must_use]
    pub fn tamanho(self) -> (f32, f32) {
        match self {
            Self::Retangulo { largura, altura } => (largura, altura),
            Self::Losango { raio } => (raio * SQRT_2, raio * SQRT_2),
        }
    }

    /// Rotação do sprite no eixo `z`, em radianos.
    #[must_use]
    pub fn rotacao(self) -> f32 {
        match self {
            Self::Retangulo { .. } => 0.0,
            Self::Losango { .. } => FRAC_PI_4,
        }
    }

    /// Converte a forma 2D na silhueta 3D correspondente.
    #[must_use]
    pub fn geometria_3d(self) -> Geometria3d {
        match self {
            Self::Retangulo { largura, altura } => Geometria3d::Caixa {
                largura,
                profundidade: altura,
            },
            Self::Losango { raio } => Geometria3d::Prisma { raio, lados: 4 },
        }
    }

    /// Testa se `ponto` está dentro da forma centrada em `centro`.
    #[must_use]
    pub fn contem(self, centro: (f32, f32), ponto: (f32, f32)) -> bool {
        let lateral = (ponto.0 - centro.0).abs();
        let vertical = (ponto.1 - centro.1).abs();
        match self {
            Self::Retangulo { largura, altura } => {
                lateral <= largura / 2.0 && vertical <= altura / 2.0
            }
            Self::Losango { raio } => lateral + vertical <= raio,
        }
    }
}

/// Uma ilha do mapa greybox.
#[derive(Clone, Copy, Debug)]
pub struct Ilha {
    /// Nome curto exibido no estado.
    pub nome: &'static str,
    /// Centro da ilha no plano `(x, y)`.
    pub centro: (f32, f32),
    /// Geometria da ilha.
    pub forma: Forma,
    /// Cor RGB com componentes entre zero e um.
    pub cor: (f32, f32, f32),
}

/// As quatro ilhas, na ordem `Topo`, `Meio`, `Inferior` e `Central`.
pub const ILHAS: [Ilha; 4] = [
    Ilha {
        nome: "Topo",
        centro: (0.0, 236.0),
        forma: Forma::Retangulo {
            largura: 360.0,
            altura: 96.0,
        },
        cor: (0.20, 0.62, 0.92),
    },
    Ilha {
        nome: "Meio",
        centro: (-320.0, 36.0),
        forma: Forma::Retangulo {
            largura: 112.0,
            altura: 248.0,
        },
        cor: (0.30, 0.82, 0.48),
    },
    Ilha {
        nome: "Inferior",
        centro: (322.0, -178.0),
        forma: Forma::Retangulo {
            largura: 196.0,
            altura: 196.0,
        },
        cor: (0.94, 0.42, 0.28),
    },
    Ilha {
        nome: "Central",
        centro: (0.0, -6.0),
        forma: Forma::Losango { raio: 104.0 },
        cor: (0.78, 0.56, 0.96),
    },
];

/// Estado puro do jogador controlado pelo cliente.
#[derive(Clone, Copy, Debug)]
pub struct EstadoJogador {
    /// Posição no plano `(x, y)`, em unidades de tela.
    pub posicao: (f32, f32),
}

/// Cria o estado inicial do jogador.
#[must_use]
pub fn estado_inicial() -> EstadoJogador {
    EstadoJogador {
        posicao: POSICAO_INICIAL,
    }
}

/// Limita uma posição à área retangular do mundo.
#[must_use]
pub fn limitar(posicao: (f32, f32)) -> (f32, f32) {
    (
        posicao.0.clamp(-LIMITE_X, LIMITE_X),
        posicao.1.clamp(-LIMITE_Y, LIMITE_Y),
    )
}

/// Aplica um deslocamento ao estado e devolve o estado já limitado.
#[must_use]
pub fn deslocar(estado: EstadoJogador, deslocamento: (f32, f32)) -> EstadoJogador {
    EstadoJogador {
        posicao: limitar((
            estado.posicao.0 + deslocamento.0,
            estado.posicao.1 + deslocamento.1,
        )),
    }
}

/// Devolve a ilha que contém `ponto`, se houver.
#[must_use]
pub fn ilha_em(ponto: (f32, f32)) -> Option<&'static Ilha> {
    ILHAS
        .iter()
        .find(|ilha| ilha.forma.contem(ilha.centro, ponto))
}

/// Converte uma posição em pixels de tela para metros da simulação.
///
/// A conversão é uma divisão por [`PIXELS_POR_METRO`], preservando a precisão
/// `f64` exigida pela comparação exata de posição dos Nós da simulação.
#[must_use]
pub fn pixels_para_metros(p: (f32, f32)) -> (f64, f64) {
    (
        f64::from(p.0) / PIXELS_POR_METRO,
        f64::from(p.1) / PIXELS_POR_METRO,
    )
}

/// Converte uma posição em metros da simulação para pixels de tela.
///
/// Inversa de [`pixels_para_metros`]: multiplica por [`PIXELS_POR_METRO`].
#[must_use]
pub fn metros_para_pixels(p: (f64, f64)) -> (f32, f32) {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "As coordenadas resultantes sao pixels de tela e cabem em f32 sem perda relevante"
    )]
    let (x, y) = (
        (p.0 * PIXELS_POR_METRO) as f32,
        (p.1 * PIXELS_POR_METRO) as f32,
    );
    (x, y)
}

/// Aproxima a posição local, em metros, de um destino da simulação quando dentro da tolerância.
///
/// Posição e destino já estão em metros (`f64`): a distância é a hipotenusa das
/// diferenças, sem conversões nem arredondamentos. O destino exato só é devolvido
/// quando a hipotenusa não excede `tolerancia_m`. Isso alinha o espelho local aos
/// Nós da simulação, cuja comparação de posição é exata (`total_cmp`, sem epsilon).
#[must_use]
pub fn aproximar_no(
    posicao_m: (f64, f64),
    destino: (f64, f64),
    tolerancia_m: f64,
) -> Option<(f64, f64)> {
    let dx = posicao_m.0 - destino.0;
    let dy = posicao_m.1 - destino.1;
    (dx.hypot(dy) <= tolerancia_m).then_some(destino)
}

#[cfg(test)]
mod testes {
    use super::{
        aproximar_no, deslocar, estado_inicial, ilha_em, metros_para_pixels, pixels_para_metros,
        Forma, ILHAS, LIMITE_X, LIMITE_Y, POSICAO_INICIAL,
    };

    #[test]
    fn estado_inicial_dentro_dos_limites() {
        let estado = estado_inicial();
        assert!((estado.posicao.0).abs() < 1e-6);
        assert!((estado.posicao.1 - POSICAO_INICIAL.1).abs() < 1e-6);
        assert!(estado.posicao.0 >= -LIMITE_X && estado.posicao.0 <= LIMITE_X);
        assert!(estado.posicao.1 >= -LIMITE_Y && estado.posicao.1 <= LIMITE_Y);
        assert_eq!(ILHAS.len(), 4);
    }

    #[test]
    fn deslocamento_respeita_os_limites() {
        let canto_superior = deslocar(estado_inicial(), (10_000.0, 10_000.0));
        assert!((canto_superior.posicao.0 - LIMITE_X).abs() < 1e-6);
        assert!((canto_superior.posicao.1 - LIMITE_Y).abs() < 1e-6);

        let canto_inferior = deslocar(estado_inicial(), (-10_000.0, -10_000.0));
        assert!((canto_inferior.posicao.0 + LIMITE_X).abs() < 1e-6);
        assert!((canto_inferior.posicao.1 + LIMITE_Y).abs() < 1e-6);
    }

    #[test]
    fn ilhas_tem_nomes_e_centros_validos() {
        assert_eq!(ILHAS[0].nome, "Topo");
        assert_eq!(ILHAS[1].nome, "Meio");
        assert_eq!(ILHAS[2].nome, "Inferior");
        assert_eq!(ILHAS[3].nome, "Central");
        assert_ne!(ILHAS[0].nome, ILHAS[3].nome);

        for ilha in &ILHAS {
            assert!(ilha.forma.contem(ilha.centro, ilha.centro));
            assert!(ilha.centro.0 >= -LIMITE_X && ilha.centro.0 <= LIMITE_X);
            assert!(ilha.centro.1 >= -LIMITE_Y && ilha.centro.1 <= LIMITE_Y);
        }
    }

    #[test]
    fn ilha_em_identifica_o_centro_e_o_vazio() {
        assert_eq!(
            ilha_em(ILHAS[3].centro).map(|ilha| ilha.nome),
            Some("Central")
        );
        assert!(ilha_em(POSICAO_INICIAL).is_none());
        assert!(ilha_em((0.0, -1000.0)).is_none());
    }

    #[test]
    fn formas_tem_tamanho_e_rotacao_coerentes() {
        let retangulo = Forma::Retangulo {
            largura: 40.0,
            altura: 20.0,
        };
        let (largura, altura) = retangulo.tamanho();
        assert!((largura - 40.0).abs() < 1e-6);
        assert!((altura - 20.0).abs() < 1e-6);
        assert_eq!(retangulo.rotacao(), 0.0);

        let losango = Forma::Losango { raio: 10.0 };
        let (lado_a, lado_b) = losango.tamanho();
        assert!((lado_a - lado_b).abs() < 1e-6);
        assert!(losango.rotacao() > 0.0);
    }

    #[test]
    fn aproximar_no_aceita_coincidencia_exata() {
        let destino: (f64, f64) = (1.5, -2.5);
        let posicao_m: (f64, f64) = (1.5, -2.5);
        assert_eq!(aproximar_no(posicao_m, destino, 2.0), Some(destino));
    }

    #[test]
    fn aproximar_no_aceita_posicao_dentro_da_tolerancia() {
        let destino: (f64, f64) = (1.5, -2.5);
        let posicao_m: (f64, f64) = (1.5, -1.0);
        assert_eq!(aproximar_no(posicao_m, destino, 2.0), Some(destino));
    }

    #[test]
    fn aproximar_no_recusa_posicao_fora_da_tolerancia() {
        let destino: (f64, f64) = (1.5, -2.5);
        let posicao_m: (f64, f64) = (10.0, -2.5);
        assert_eq!(aproximar_no(posicao_m, destino, 2.0), None);
    }

    #[test]
    fn pixels_para_metros_converte_velocidade_conhecida() {
        assert_eq!(pixels_para_metros((260.0, 0.0)), (2.6, 0.0));
    }

    #[test]
    fn metros_para_pixels_inverte_pixels_para_metros() {
        let p: (f32, f32) = (260.0, -236.0);
        let (x, y) = metros_para_pixels(pixels_para_metros(p));
        assert!((f64::from(x) - f64::from(p.0)).abs() < 1e-6);
        assert!((f64::from(y) - f64::from(p.1)).abs() < 1e-6);
    }
}
