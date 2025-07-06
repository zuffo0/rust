use std::io;
use std::fs::File;
use std::io::Write;
use std::io::Read;

struct Personagem {
    nome: String,
    vida: u32,
    velocidade: u32,
    forca: u32,
}

fn main() {
    {
    let mut name: String = "".to_string();
    let mut save = File::create("save").expect("failed at creating save file");
    println!("{:-^40}", "RPG");
    println!("Digite seu nickname:");
    io::stdin().read_line(&mut name);
    name = name.trim().to_string();
    let character = Personagem {
        nome: name,
        vida: 100,
        velocidade: 10,
        forca: 10,
    };
    let save_str: String = format!("{},100,10,10",character.nome);
    // formato do save seguindo o struct
    save.write(save_str.as_bytes()).expect("write failed");
}
}


fn readsave(){
    let mut data_file = File::open("data.txt").unwrap();
    let mut file_content = String::new();
    let mut infos: Vec<String> = Vec::new();
}

