pub fn exemplo() {
    funcao_basica();

    funcao_com_parametros(10, 20);

    println!("Função com retorno");
    let mut valor = funcao_com_retorno();
    println!("valor => {}", valor);

    println!("Função com parâmetro e retorno");
    valor = funcao_com_parametros_e_retorno(10, 15);
    println!("valor => {}", valor);
}

fn funcao_basica() {
    println!("Função básica");
}

fn funcao_com_parametros(a: u32, b: u32) {
    println!("Função com Parâmetro");
    println!("a, b => {}, {}", a, b);
}

fn funcao_com_retorno() -> i32 {
    10
}

fn funcao_com_parametros_e_retorno(a: i32, b: i32) -> i32 {
    a + b
}