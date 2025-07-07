use std::io;

const BANNER: &str = r"
_________        .__               .__              .___                   
\_   ___ \_____  |  |   ____  __ __|  | _____     __| _/________________   
/    \  \/\__  \ |  | _/ ___\|  |  \  | \__  \   / __ |/  _ \_  __ \__  \  
\     \____/ __ \|  |_\  \___|  |  /  |__/ __ \_/ /_/ (  <_> )  | \// __ \_
 \______  (____  /____/\___  >____/|____(____  /\____ |\____/|__|  (____  /
        \/     \/          \/                \/      \/                 \/ 
";

const VERSION: &str = "1.0.0R";

const OPTIONS: &str = r"
[1] Somar
[2] Subtrair
[3] Multiplicar
[4] Dividir
";

fn clear() {
    print!("\x1b[2J\x1b[H");
}

struct Calculos {
    v1: i128,
    v2: i128,
}

impl Calculos {
    fn somar(&self) -> i128 {
        return self.v1 + self.v2;
    }

    fn subtrair(&self) -> i128 {
        return self.v1 - self.v2;
    }

    fn multiplicar(&self) -> i128 {
        let v1 = self.v1 as i128;
        let v2 = self.v2 as i128;
        return v1 * v2;
    }

    fn dividir(&self) -> i128 {
        return self.v1 / self.v2;
    }
}

fn main() {
    loop {
        let mut opcao: String = "".to_string();
        let mut n1str: String = "".to_string();
        let mut n2str: String = "".to_string();
        clear();
        println!("{BANNER}\nVersão {VERSION}");
        println!("Insira o número 1:");
        io::stdin()
            .read_line(&mut n1str)
            .expect("Erro ao pegar input n1 do usuário.");
        println!("\nInsira o número 2:");
        io::stdin()
            .read_line(&mut n2str)
            .expect("Erro ao pegar input n2 do usuário.");

        //conversão de string p i128
        let n1str = n1str.trim();
        let n2str = n2str.trim();
        let n1: i128 = n1str.parse().expect("erro na conversão do n1");
        let n2: i128 = n2str.parse().expect("erro na conversão do n2");

        let calc = Calculos { v1: n1, v2: n2 };

        println!("Selecione uma das opções abaixo:");
        println!("{OPTIONS}\n>");
        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao pegar input da operação desejada.");
        opcao = opcao.trim().to_string();

        let resultado: i128 = match opcao.as_str() {
            "1" => calc.somar(),
            "2" => calc.subtrair(),
            "3" => calc.multiplicar(),
            "4" => calc.dividir(),
            _ => 0,
        };
        println!("O resultado é: {resultado}\n");
        println!("Clique enter para fazer outro cálculo.");
        {
            let mut ignore: String = String::new();
            let _ = io::stdin().read_line(&mut ignore);
        }
    }
}
