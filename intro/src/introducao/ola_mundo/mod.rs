mod comentario;
mod impressao;

use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    
    loop {
        let itens: [&str; 7] = [
            "Comentário",
            "Impressão",
            "Impressão Formatada",
            "Impressão Formatada - Depurar",
            "Impressão Formatada - Exibição",
            "Impressão Formatada - Testcase Lista",
            "Impressão Formatada - Formatação",
        ];
    
        let selecionado = exibir_menu("Olá Mundo", &itens, false);
    
        limpar_tela();
        match selecionado {
            1 => comentario::exemplo(),
            2 => impressao::exemplo(),
            3 => impressao::exemplo_formatada(),
            4 => impressao::exemplo_formatada_depurar(),
            5 => impressao::exemplo(),
            6 => impressao::exemplo(),
            7 => impressao::exemplo(),
            _ => break,
        }

        esperar_enter();
    }
}