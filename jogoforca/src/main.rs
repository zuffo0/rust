use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Read;

const VERSION: &str = "0.0.1A"; //alpha version

const VIDAS: [&str; 7] = [
    r"
    ___________
    |         |
    |         
    |
    |
    |
----------
",
    r"
    ___________
    |         |
    |         o
    |
    |
    |
----------
",
    r"
    ___________
    |         |
    |         o
    |         |
    |
    |
----------
",
    r"
    ___________
    |         |
    |         o
    |         |\
    |
    |
----------
",
    r"
    ___________
    |         |
    |         o
    |        /|\
    |
    |
----------
",
    r"
    ___________
    |         |
    |         o
    |        /|\
    |        /
    |
----------
",
    r"
    ___________
    |         |
    |         o
    |        /|\
    |        / \
    |
----------
",
];

fn clear() {
    print!("\x1b[2J\x1b[H");
}

fn main() {
    //selecionar uma palavra aleátoria
    let mut file = File::open("palavras.txt").unwrap();
    let mut palavras: Vec<&str> = Vec::new();
    let mut palavrasraw: String = String::new();
    file.read_to_string(&mut palavrasraw)
        .expect("erro ao escrever palavras no vetor");
    palavras = palavrasraw.split(',').collect();
    let palavra_aleatoria: usize = rand::thread_rng()
        .gen_range(1..=palavras.len())
        .try_into()
        .unwrap();
    let palavra_aleatoria: &str = palavras[palavra_aleatoria];
    let palavra_aleatoria: &str = palavra_aleatoria.trim();
    let tamanhopalavra: usize = palavra_aleatoria.chars().count().try_into().unwrap();
    let mut tracejado: Vec<char> = Vec::new();
    for _l in 0..tamanhopalavra {
        tracejado.push('_');
    }

    // jogo
    let mut vida: usize = 0;
    loop {
        //variaveis
        let mut letraescolhida: char = '.';
        let mut tracejadostrjunto: String = String::new();
        let mut tracejadostr: String = String::new();
        let mut letraescolhidastr: String = String::new();
        let mut letrascertas_pos: Vec<usize> = Vec::new();
        clear();

        //criar as strings p mostrar pro jogador e verificar se a palavra está certa
        for l in &tracejado {
            let letra: String = format!("{} ", l);
            tracejadostr.push_str(&letra);
            tracejadostrjunto.push_str(&l.to_string());
        }
        tracejadostrjunto = tracejadostrjunto.trim().to_string();

        //verificação p ver se perdeu
        println!("{}", VIDAS[vida]);
        if vida == 6 {
            println!("Você perdeu!");
            println!("A palavra sorteada era: {palavra_aleatoria}");
            break;
        }

        println!("{tracejadostr}"); //printa o progresso atual

        //verificação pra ver se ganhou
        if tracejadostrjunto == palavra_aleatoria {
            println!("Párabens! Você acertou a palavra!");
            break;
        }
        //inserir letra
        println!("Insira uma letra:");
        io::stdin()
            .read_line(&mut letraescolhidastr)
            .expect("falha ao pegar input do usuário");
        //transformar em char
        letraescolhidastr = letraescolhidastr.trim().to_string();
        letraescolhida = letraescolhidastr
            .parse()
            .expect("erro ao transformar string para char");

        //transformar em lowercase
        let letraescolhida = letraescolhida.to_lowercase().next().unwrap();

        //verificar se a letra dada está na palavra
        let mut pos: usize = 0;
        for l in palavra_aleatoria.chars() {
            pos += 1;
            if letraescolhida == l {
                letrascertas_pos.push(pos);
                tracejado[pos - 1] = l;
            }
        }

        // se não acertou nada, tira uma vida
        if letrascertas_pos.len() < 1 {
            vida += 1;
        }
    }
}
//amanha adiciono o readme dessabomba