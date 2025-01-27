mod utils;
mod rust; 
mod coder;

use std::process::exit;

use crate::utils::terminal::{exibir_menu,limpar_tela};
use crate::rust::executar_rust;
use crate::coder::executar_coder;

fn main() {
    loop {
        let itens: [&str; 2] = [
            "RUST",
            "COD3R"
        ];
    
        let selecionado: u32 = exibir_menu("Principal", &itens, true);

        limpar_tela();
        match selecionado {
            1 => executar_rust(),
            2 => executar_coder(),
            _ => exit(0)
        }
    }
} 