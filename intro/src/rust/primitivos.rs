use crate::utils::terminal::{esperar_enter, exibir_menu, limpar_tela};

pub fn executar() {
    loop {
        let itens = [
            "Escalar",
            "Composto",
            "Variáveis"
        ];

        let selecionado = exibir_menu("Tipos Primitivos", &itens, false);

        limpar_tela();
        match selecionado {
            1 => exemplo_escalar(),
            2 => exemplo_composto(),
            3 => exemplo_variaveis(),
            _ => break,
        }
    }
}

fn exemplo_escalar() {
    println!("Inteiros\n");
    println!("\tAssinados:");
    println!("\t\t- i8 i16 i32, i64, i128 e isize");
    println!("\tNão Assinados:");
    println!("\t\t- u8 u16 u32, u64, u128 e usize\n\n");

    println!("Ponto Flutuante");
    println!("f32, f64\n\n");

    println!("Char\n");
    println!("\tUnicode (4bytes cada)");
    println!("\t\t- 'a', 'α', '∞'\n\n");

    println!("Booleano");
    println!("true ou false\n\n");

    println!("O tipo de unidade");
    println!("Único valor possível - () tupla vazia.");

    esperar_enter();
}

fn exemplo_composto() {
    let my_array = [1,2,3];
    let my_tuple = (1, true);
    println!("Arrays - {:?}\n", my_array);
    println!("Tuplas - {:?}\n", my_tuple);

    esperar_enter();
}

fn exemplo_variaveis() {
    // Variáveis podem ter o tipo anotado
    let logico: bool = true;
    println!("Tipo anotado => {}", logico);

    let a_float: f64 = 1.0; // Anotação Regular
    let an_integer = 5i32; // Anotação de sufixo

    println!("Anotação Regular e Sufixo => {} {}", a_float, an_integer);

    // Valores tipados por padrão
    let default_float = 3.2; // `f64`
    let default_integer = 7; // `i32`

    println!("Tipo padrão float (f64), inteiro (i32) => {} {}", default_float, default_integer);

    // O tipo pode ser inferido conforme o contexto
    let mut inferred_type = 12; // Tipo i64 é inferido de outra linha
    println!("Tipo {} (i32) inferido em outra linha => ", inferred_type);
    inferred_type = 4294967296i64;
    println!("{} (i64) a partir de agora", inferred_type);

    // A mutabilidade permite que o valor da variável seja mudado
    let mut mutable = 12;
    print!("Variável mutável antes e depois => {}", mutable); 
    mutable = 21;
    println!(" {}", mutable);

    // O tipo não é mutável
    // mutable = true;

    esperar_enter();
}