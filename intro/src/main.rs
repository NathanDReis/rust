mod utils;

mod fundamentos;
mod tipos;
mod controle;
mod funcoes;
mod ownership;

use std::process::exit;
use utils::terminal::{exibir_menu, limpar_tela};

fn main() {
    loop {
        let itens: [&str; 5] = [
            "Fundamentos",
            "Tipos",
            "Controle",
            "Funções",
            "Qwnership",
        ];
    
        let selecionado: u32 = exibir_menu("Principal", &itens, true);

        limpar_tela();
        match selecionado {
            1 => fundamentos::executar(),
            2 => tipos::executar(),
            3 => controle::executar(),
            4 => funcoes::executar(),
            5 => ownership::executar(),
            _ => exit(0)
        }
    }
} 