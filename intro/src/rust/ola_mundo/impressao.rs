// >>>>>>>>>>>>>> Impressões >>>>>>>>>>>>>>

pub fn exemplo() {
    println!("\nExemplos\n");

    println!("format! => texto formatado para String");
    println!("print! => é um format! escrito no console (io::stdout)");
    println!("println! => é um print! com uma linha anexada");
    println!("eprint! => é um print! para erro (io::stderr)");
    println!("eprintln! => é um println! para erro");
}




// >>>>>>>> Impressões Formatadas >>>>>>>>>>>>>>

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




// >>>>>>>>>>>>>> Debug >>>>>>>>>>>>>>

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




// >>>>>>>>>>>>>> Exibir Debug >>>>>>>>>>>>>>

use std::fmt;

// struct Structure(i32);

// // Para qualquer tipo não genérico fmt::Display 
// // Se for um tipo genérico Vec<T> use fmt::Debug 
// impl fmt::Display for Structure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

#[derive(Debug)]
struct MinMax(i64, i64);

// Implementar display para MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Definindo a estrutura
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Da mesma forma implemente `Display` para `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn exemplo_formatada_exibir() {
    let minmax = MinMax(0, 14);

    println!("Comparar estruturas:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    println!("Debug Format: {:#?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("O grande range é {big} e o pequeno é {small}",
        small = small_range,
        big = big_range,
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    
    println!("Comparar pontos:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}



// >>>>>>>>>>>>>> Impressão de Lista >>>>>>>>>>>>>>

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

pub fn exemplo_formatada_testcase_lista() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

// >>>>>>>>>>>>>> Formatação >>>>>>>>>>>>>>

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 's' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'w' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c    
        )
    }
}

#[derive(Debug)]
struct Color {
    _red: u8,
    _green: u8,
    _blue: u8,
}

pub fn exemplo_formatada_formatacao() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }

    for color in [
        Color { _red: 128, _green: 255, _blue: 90 },
        Color { _red: 0, _green: 3, _blue: 254 },
        Color { _red: 0, _green: 0, _blue: 0 },
    ] {
        println!("{:?}", color);
    }
}