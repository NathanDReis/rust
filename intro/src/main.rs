mod utils;

mod introducao; 
mod fundamentos;
mod tipos;
mod controle;
mod funcoes;
mod ownership;

use std::process::exit;
use utils::terminal::{exibir_menu, limpar_tela};

fn main() {
    loop {
        let itens: [&str; 6] = [
            "Introdução",
            "Fundamentos",
            "Tipos",
            "Controle",
            "Funções",
            "Qwnership",
        ];
    
        let selecionado: u32 = exibir_menu("Principal", &itens, true);

        limpar_tela();
        match selecionado {
            1 => introducao::executar(),
            2 => fundamentos::executar(),
            3 => tipos::executar(),
            4 => controle::executar(),
            5 => funcoes::executar(),
            6 => ownership::executar(),
            _ => exit(0)
        }
    }
} 