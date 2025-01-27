mod fns;
mod lambda;

use crate::utils::terminal::{exibir_menu, limpar_tela, esperar_enter};

pub fn executar() {
    loop {
        let itens= [
            "Início",
            "Lambda - Map",
            "Lambda - Filter",
        ];
    
        let selecionado = exibir_menu("Funções", &itens, false);
    
        limpar_tela();
        match selecionado {
            1 => fns::exemplo(),
            2 => lambda::exemplo_map(),
            3 => lambda::exemplo_filter(),
            _ => break,
        }
    
        esperar_enter();
    }
}