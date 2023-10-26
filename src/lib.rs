use std::io;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Jogador {
    X,
    O,
}

impl Jogador {
    fn outro(&self) -> Jogador {
        match self {
            Jogador::X => Jogador::O,
            Jogador::O => Jogador::X,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Jogador::X => String::from("X"),
            Jogador::O => String::from("O"),
        }
    }
}

enum Estado {
    Jogando {
        rodada: u8,
    },
    Finalizado {
        vencedor: Option<Jogador>,
        combinacao_vitoriosa: Option<[usize; 3]>,
    },
}

pub struct Partida {
    tabuleiro: [Option<Jogador>; 9],
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

    pub fn jogar(&mut self) {
        while let Estado::Jogando { rodada: _ } = self.estado {
            self.mostrar_tabuleiro(None);
            self.pedir_movimento();
            if let None = self.verificar_vencedor() {
                self.proxima_rodada();
            }
        }

        if let Estado::Finalizado {
            vencedor,
            combinacao_vitoriosa,
        } = &self.estado
        {
            if let Some(jogador) = vencedor {
                self.mostrar_tabuleiro(*combinacao_vitoriosa);
                println!("Jogador {:?} venceu!", jogador);
            } else {
                self.mostrar_tabuleiro(None);
                println!("Empate: Jogo deu velha!");
            }
        }
    }

    fn mostrar_tabuleiro(&self, destacar: Option<[usize; 3]>) {
        limpar_tela();
        for (i, jogador) in self.tabuleiro.iter().enumerate() {
            let cor;
            if let None = jogador {
                print!("|{}", (i + 1).to_string());
            } else {
                cor = if let Some(p) = destacar {
                    if p.contains(&i) {
                        "\x1b[32m" // Verde
                    } else {
                        "\x1b[33m" // Amarelo
                    }
                } else {
                    "\x1b[33m" // Amarelo
                };
                print!("|");
                print_colorido(&jogador.as_ref().unwrap().to_string(), cor);
            }

            if (i + 1) % 3 == 0 {
                println!("|");
            }
        }
    }

    fn pedir_movimento(&mut self) {
        loop {
            println!("\nDigite a posição desejada conforme o tabuleiro: ");

            let mut entrada = String::new();
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler entrada");

            match entrada.trim().parse::<usize>() {
                Ok(pos @ 1..=9) => {
                    if let Some(_) = &self.tabuleiro[pos - 1] {
                        println!("Já existe uma marcação na posição especificada!");
                    } else {
                        self.tabuleiro[pos - 1] = Some(self.jogador_atual);
                        break;
                    }
                }
                _ => println!("Posição inválida: Digite um valor entre 1 e 9!"),
            }
    
        }
    }

    fn verificar_vencedor(&mut self) -> Option<Jogador> {
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
            if let Some(jogador) = &self.tabuleiro[a] {
                if self.tabuleiro[a] == self.tabuleiro[b] && self.tabuleiro[b] == self.tabuleiro[c]
                {
                    self.estado = Estado::Finalizado {
                        combinacao_vitoriosa: Some([a, b, c]),
                        vencedor: Some(*jogador),
                    };
                    return Some(*jogador);
                }
            }
        }

        return None;
    }

    fn proxima_rodada(&mut self) {
        if let Estado::Jogando { ref mut rodada } = self.estado {
            if *rodada < 9 {
                *rodada += 1;
                self.jogador_atual = self.jogador_atual.outro();
            } else {
                self.estado = Estado::Finalizado {
                    vencedor: None,
                    combinacao_vitoriosa: None,
                }
            }
        }
    }
}

fn print_colorido(texto: &str, cor_ansi: &str) {
    print!("{}{}{}", cor_ansi, texto, "\x1b[0m");
}

fn limpar_tela() {
    print!("\x1B[2J\x1B[1;1H");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nenhum_vencedor() {
        let mut p = Partida::new();
        assert!(p.verificar_vencedor().is_none());
    }

    #[test]
    fn vencedor_linha() {
        let mut p = Partida::new();
        p.tabuleiro = [
            Some(Jogador::O),
            Some(Jogador::O),
            Some(Jogador::O),
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        assert_eq!(p.verificar_vencedor(), Some(Jogador::O));
    }

    #[test]
    fn vencedor_coluna() {
        let mut p = Partida::new();
        p.tabuleiro = [
            Some(Jogador::X),
            None,
            None,
            Some(Jogador::X),
            None,
            None,
            Some(Jogador::X),
            None,
            None,
        ];
        assert_eq!(p.verificar_vencedor(), Some(Jogador::X));
    }

    #[test]
    fn vencedor_diagonal() {
        let mut p = Partida::new();
        p.tabuleiro = [
            Some(Jogador::X),
            Some(Jogador::O),
            Some(Jogador::X),
            Some(Jogador::O),
            Some(Jogador::X),
            Some(Jogador::O),
            Some(Jogador::O),
            Some(Jogador::X),
            Some(Jogador::X),
        ];
        assert_eq!(p.verificar_vencedor(), Some(Jogador::X));
    }

    #[test]
    fn empate() {
        let mut p = Partida::new();
        p.tabuleiro = [
            Some(Jogador::O),
            Some(Jogador::O),
            Some(Jogador::X),
            Some(Jogador::X),
            Some(Jogador::X),
            Some(Jogador::O),
            Some(Jogador::O),
            Some(Jogador::X),
            Some(Jogador::X),
        ];
        assert_eq!(p.verificar_vencedor(), None);
    }
}
