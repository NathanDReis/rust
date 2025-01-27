mod ola_mundo;
mod primitivos;

use crate::utils::terminal::{exibir_menu,limpar_tela};

pub fn executar_rust() {
   loop {
       let itens: [&str; 24] = [
        "Olá, Mundo!",
        "Primitivos",
        "Tipos Personalizados",
        "Ligações de Variáveis",
        "Tipos",
        "Conversão",
        "Expressões",
        "Fluxo de Controle",
        "Funções",
        "Módulos",
        "Crates",
        "Cargo",
        "Atributos",
        "Genéricos",
        "Regras de Escopo",
        "Traits",
        "Macro",
        "Tratamento de Erro",
        "Tipos de Bibliotecas std",
        "Std misc",
        "Testes",
        "Operações Inseguras",
        "Compatibilidade",
        "Meta"
       ];

       let selecionado: u32 = exibir_menu("RUST", &itens, false);

       limpar_tela();
       match selecionado {
           1 => ola_mundo::executar(),
           2 => primitivos::executar(),
           3 => println!("Exemplo"),
           4 => println!("Exemplo"),
           5 => println!("Exemplo"),
           6 => println!("Exemplo"),
           7 => println!("Exemplo"),
           8 => println!("Exemplo"),
           9 => println!("Exemplo"),
           10 => println!("Exemplo"),
           11 => println!("Exemplo"),
           12 => println!("Exemplo"),
           13 => println!("Exemplo"),
           14 => println!("Exemplo"),
           15 => println!("Exemplo"),
           16 => println!("Exemplo"),
           17 => println!("Exemplo"),
           18 => println!("Exemplo"),
           19 => println!("Exemplo"),
           20 => println!("Exemplo"),
           21 => println!("Exemplo"),
           22 => println!("Exemplo"),
           23 => println!("Exemplo"),
           24 => println!("Exemplo"),
           _ => break,
       }
   } 
}