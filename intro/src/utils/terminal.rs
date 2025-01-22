use std::io;

fn exibir_itens(itens: &[&str]) {
    for (indice, item) in itens.iter().enumerate() {
        println!("{} - {}", indice + 1, item);
    }
}

pub fn exibir_menu(titulo: &str, itens: &[&str], sair: bool) -> u32 {
    limpar_tela();

    let completo: String = String::from("Kings Rust :: ") + titulo;
    println!("{}", completo);
    println!("{}", "=".repeat(completo.len()));

    exibir_itens(itens);
    println!("{}", if sair { "* - Sair" } else { "* - Voltar" });

    println!("\nEscolha uma opção: ");
    let mut linha: String = String::new();
    io::stdin().read_line(&mut linha).expect("Failed to read line");

    let opcao: Result<u32, _> = linha.trim().parse();

    match opcao {
        Ok(opcao) => opcao,
        _=> 0,
    }
} 

pub fn esperar_enter() {
    println!("\nPressione Enter para continuar...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
}

pub fn limpar_tela() {
    print!("{esc}c", esc = 27 as char);
}