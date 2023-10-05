#[derive(Debug, PartialEq, Eq)]
enum Casa {
    Vazia,
    X,
    O
}

#[derive(Debug)]
enum Jogador {
    X,
    O
}

impl Jogador {
    fn outro(&self) -> Jogador {
        match self {
            Jogador::X => Jogador::O,
            Jogador::O => Jogador::X
        }
    }
 }


enum Estado {
    Jogando,
    Finalizado(Jogador)
}

struct Partida {
    tabuleiro: [Casa; 9],
    jogador_atual: Casa,
    rodada: u8,
    estado: Estado
 }

impl Partida {
    fn reiniciar(&mut self) {
        let nova_partida = Partida::new();
        self.tabuleiro = nova_partida.tabuleiro;
        self.jogador_atual = nova_partida.jogador_atual;
        self.rodada = nova_partida.rodada;
        self.estado = nova_partida.estado;
    }

    fn new() -> Partida {
        Partida { 
            tabuleiro: [
                Casa::Vazia, Casa::Vazia, Casa::Vazia, 
                Casa::Vazia, Casa::Vazia, Casa::Vazia, 
                Casa::Vazia, Casa::Vazia, Casa::Vazia, 
            ], 
            jogador_atual: Casa::X, 
            rodada: 1, 
            estado: Estado::Jogando
        }
    }
 }


fn main() {
    let partida = Partida::new();
    loop {


        // Reiniciar a partida
        if let Estado::Finalizado(vencedor) = partida.estado {
            println!("O vencedor foi o jogador: {:?}", vencedor);
            break;
        }
    }

    



    
}

fn campos_sequenciais(a: &Casa, b: &Casa, c: &Casa) -> bool {
    a != &Casa::Vazia && a == b && b == c
}

fn verificar_vencedor(tabuleiro: &[Casa]) -> Option<Casa> {
    // Retornar campos vencedores depois
    match tabuleiro {
        // ! Linhas
        [a, b, c, ..] if campos_sequenciais(a, b, c) => 
            println!("Vecedor na linha 1: {:?}", a),
        [_, _, _, a, b, c, ..] if campos_sequenciais(a, b, c) => 
            println!("Vencedor na linha 2: {:?}", a),
        [.., a, b, c] if campos_sequenciais(a, b, c) => 
            println!("Vencedor na linha 3: {:?}", a),
        // ! Colunas
        [a, _, _, b, _, _, c, ..] if campos_sequenciais(a, b, c) => 
            println!("Vencedor na coluna 1: {:?}", a),
        [_, a, _, _, b, _, _, c, ..] if campos_sequenciais(a, b, c) => 
            println!("Vencedor na coluna 2: {:?}", a),
        [.., a, _, _, b, _, _, c] if campos_sequenciais(a, b, c) => 
            println!("Vencedor na coluna 3: {:?}", a),
        // ! Diagonais
        [a, _, _, _, b, _, _, _, c] if campos_sequenciais(a, b, c) =>
            println!("Vencedor na diagonal 1: {:?}", a),
        [_, _, a, _, b, _, c, ..] if campos_sequenciais(a, b, c) =>
            println!("Vencedor na diagonal 2: {:?}", a),
        _ => println!("Nenhum vencedor")
    };
    Some(Casa::O)
}