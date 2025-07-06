use std::io;

const BANNER: &str = r"
_________        .__               .__              .___                   
\_   ___ \_____  |  |   ____  __ __|  | _____     __| _/________________   
/    \  \/\__  \ |  | _/ ___\|  |  \  | \__  \   / __ |/  _ \_  __ \__  \  
\     \____/ __ \|  |_\  \___|  |  /  |__/ __ \_/ /_/ (  <_> )  | \// __ \_
 \______  (____  /____/\___  >____/|____(____  /\____ |\____/|__|  (____  /
        \/     \/          \/                \/      \/                 \/ 
";

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
    v1: i32,
    v2: i32,
}

impl Calculos {
    fn somar(&self) -> i32 {
        return self.v1 + self.v2;
    }

    fn subtrair(&self) -> i32 {
        return self.v1 - self.v2;
    }

    fn multiplicar(&self) -> i64 {
        let v1 = self.v1 as i64;
        let v2 = self.v2 as i64;
        return v1 * v2;
    }

    fn dividir(&self) -> i32 {
        return self.v1 / self.v2;
    }
}

fn main() {
    loop {
        let mut opcao: String = "".to_string();
        let mut n1str: String = "".to_string();
        let mut n2str: String = "".to_string();
        clear();
        println!("{BANNER}\n");
        println!("Insira o numero 1:");
        io::stdin().read_line(&mut n1str);
        println!("\nInsira o numero 2:");
        io::stdin().read_line(&mut n2str);

        //conversão de string p i32
        let n1str = n1str.trim();
        let n2str = n2str.trim();
        let n1: i32 = n1str.parse().expect("erro na conversão do n1");
        let n2: i32 = n2str.parse().expect("erro na conversão do n2");

        let calc = Calculos { v1: n1, v2: n2 };

        println!("Selecione uma das opções abaixo:");
        println!("{OPTIONS}\n>");
        io::stdin().read_line(&mut opcao);
        opcao = opcao.trim().to_string();

        let resultado: i64 = match opcao.as_str() {
            "1" => calc.somar() as i64,
            "2" => calc.subtrair() as i64,
            "3" => calc.multiplicar() as i64,
            "4" => calc.dividir() as i64,
            _ => 0,
        };
        println!("O resultado é: {resultado}");
        break;
    }
}
