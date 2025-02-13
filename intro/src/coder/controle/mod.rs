mod condicionais;
mod loops;

use crate::utils::terminal::{exibir_menu, limpar_tela, esperar_enter};

pub fn executar() {
    loop {
        let itens= [
            "Condicionais",
            "Loop - For Range",
            "Loop - Array",
            "Loop - While",
            "Loop - Loop",
        ];
    
        let selecionado = exibir_menu("Controle", &itens, false);
    
        limpar_tela();
        match selecionado {
            1 => condicionais::exemplo(),
            2 => loops::exemplo_for_range(),
            3 => loops::exemplo_for_array(),
            4 => loops::exemplo_while(),
            5 => loops::exemplo_loop(),
            _ => break,
        }
    
        esperar_enter();
    }
}