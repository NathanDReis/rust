pub fn exemplo() {
    println!("\nExemplos\n");

    println!("format! => texto formatado para String");
    println!("print! => é um format! escrito no console (io::stdout)");
    println!("println! => é um print! com uma linha anexada");
    println!("eprint! => é um print! para erro (io::stderr)");
    println!("eprintln! => é um println! para erro");
}

pub fn exemplo_formatada() {
    // {} PARA o USUÁRIO
    // {:?} PARA o DEPURAÇÃO

    println!("{} dias", 31);
    println!("{1}, essa é a {0}. {0}, esse é o {1}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
        object = "o cachorro preguiçoso",
        subject = "a rápida raposa marrom",
        verb = "salta sobre"
    );

    println!("\nBases\n");
    println!("Base 10           => {}", 69420); // 69420
    println!("Base 2            => {:b}", 69420); // 10000111100101100
    println!("Base 8            => {:o}", 69420); // 207454
    println!("Base 16           => {:x}", 69420); // 10f2c

    println!("\nEspaços (Paid)\n");
    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<width$}", number = 1, width = 5);

    println!("\nPaid com format!\n");
    let txt: String = format!("{:0>5}", 1);
    println!("{}", txt);

    // O Rust checa o número de argumentos usados
    // println!("My name is {0}, {1} {0}", "Bond");
}

#[derive(Debug)]
struct Estrutura(i32);

pub fn exemplo_formatada_depurar() {
    // Printar com `{:?}` é semelhante ao `{}`
    println!("{:?} meses no ano.", 12);
    println!("{1:?} {0:?} é o nome do {actor:?}",
        "Slater",
        "Christian",
        actor="ator"
    );

    // Permite printar estruturas
    println!("Agora a estrutura {:?} pode ser printada\n", Estrutura(3));

    // Para ter mais estilo
    #[derive(Debug)]
    struct Pessoa<'a> {
        nome: &'a str,
        idade: u8
    }

    let idade: u8 = 37;
    let roberto = Pessoa { nome: "Roberto", idade};

    println!("{:#?} \nNome: {}\nIdade: {}", roberto, roberto.nome, roberto.idade);
}