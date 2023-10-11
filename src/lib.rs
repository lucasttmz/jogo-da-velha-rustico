#[derive(Debug, PartialEq, Eq)]
enum Casa {
    X,
    O,
}

pub struct Config {
    melhor_de: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        
        let melhor_de: u32 = match args.next() {
            Some(arg) => arg.parse().expect("Quantidade de partidas devem ser um número"),
            None => 1
        };

        return Ok(Config { melhor_de });
    }
}

#[derive(Debug)]
enum Jogador {
    X,
    O,
}

impl Jogador {
    fn casa(&self) -> Casa {
        match self {
            Jogador::X => Casa::X,
            Jogador::O => Casa::O,
        }
    }

    fn outro(&self) -> Jogador {
        match self {
            Jogador::X => Jogador::O,
            Jogador::O => Jogador::X,
        }
    }
}

enum Estado {
    Jogando {
        rodada: u8,
    },
    Finalizado {
        vencedor: Jogador,
        combinacao_vitoriosa: [usize; 3],
    },
}

pub struct Partida {
    tabuleiro: [Option<Casa>; 9],
    jogador_atual: Jogador,
    estado: Estado,
}

impl Partida {
    pub fn new() -> Partida {
        Partida {
            tabuleiro: [None, None, None, None, None, None, None, None, None],
            jogador_atual: Jogador::X,
            estado: Estado::Jogando { rodada: 1 },
        }
    }

    fn reiniciar(&mut self) {
        *self = Partida::new();
        self.jogar();
    }

    pub fn jogar(&mut self) {}

    fn verificar_vencedor(&mut self) -> Option<&Casa> {
        // Retornar campos vencedores depois
        let combinacoes_vitoria = [
            // * Linhas
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // * Colunas
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // * Diagonais
            [0, 4, 8],
            [2, 4, 6],
        ];

        for combinacao in &combinacoes_vitoria {
            let [a, b, c] = [combinacao[0], combinacao[1], combinacao[2]];
            if let Some(casa) = &self.tabuleiro[a] {
                if self.tabuleiro[a] == self.tabuleiro[b] && self.tabuleiro[b] == self.tabuleiro[c]
                {
                    let vencedor = match casa {
                        Casa::X => Jogador::X,
                        Casa::O => Jogador::O,
                    };

                    self.estado = Estado::Finalizado {
                        combinacao_vitoriosa: [a, b, c],
                        vencedor,
                    };
                    return Some(casa);
                }
            }
        }

        return None;
    }
}
