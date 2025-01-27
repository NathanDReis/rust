mod fundamentos;
mod tipos;
mod controle;
mod funcoes;
mod ownership;

use crate::utils::terminal::{exibir_menu,limpar_tela};

pub fn executar_coder() {
   loop {
       let itens: [&str; 5] = [
        "Fundamentos",
        "Tipos",
        "Controle",
        "Funções",
        "Qwnership",
       ];

       let selecionado: u32 = exibir_menu("COD3R", &itens, false);

       limpar_tela();
       match selecionado {
        1 => fundamentos::executar(),
        2 => tipos::executar(),
        3 => controle::executar(),
        4 => funcoes::executar(),
        5 => ownership::executar(),
           _ => break,
       }
   } 
}