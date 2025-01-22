pub fn exemplo() {
    let ativo: bool = true; // false
    println!("Booleano {}", ativo);

    let caractere: char = 'a';
    println!("Caractere {}", caractere);

    let nome: &str = "Maria Silva";
    println!("string {}", nome);

    let mut nome: String = String::from("Nathan David");
    nome.push_str(" Reis");
    println!("String {}", nome);

    // i8, i16, i32 (default), i64, i128, isize (Segue a arquitetura do computador)
    // u8, u16, u32, u64, u128, usize
    let quantidade: i32 = 10;
    println!("Inteiro {}", quantidade);

    // f32, f64 (defaut)
    let preco: f64 = 10.99;
    println!("Ponto flutuante {}", preco);
}