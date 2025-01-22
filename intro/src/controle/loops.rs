pub fn exemplo_for_range() {
    // De 1 a 4
    println!("De 1 a 4");
    for a in 1..5 {
        println!("a => {}", a);
    }
    
    // De 1 a 5
    println!("De 1 a 5");
    for b in 1..=5 {
        println!("b => {}", b);
    }
    
    // De 1 a 4 invertido (reverso)
    println!("De 1 a 4 reverso");
    for c in (1..5).rev() {
        println!("c => {}", c);
    }
    
    // De 1 ao 10 contando de 2 em 2
    println!("De 1 a 10 de 2 em 2");
    for d in (1..=10).step_by(2) {
        println!("d => {}", d);
    }
}

pub fn exemplo_for_array() {
    let array = [1, 2, 3, 4, 5];

    println!("Percorrendo as posições");
    for i in 0..array.len() {
        println!("array[{}]", i);
    }

    println!("Percorrendo os valores");
    for valor in array {
        println!("valor => {}", valor);
    }

    println!("Percorrendo as posições e os valores");
    for (i, valor) in array.iter().enumerate() {
        println!("array[{}] => {}", i, valor);
    }
}

pub fn exemplo_while() {
    let mut a = 1;

    while a <= 5 {
        println!("a => {}", a);
        a += 1;
    }
}

pub fn exemplo_loop() {
    let mut b = 1;

    loop {
        println!("b => {}", b);
        b += 1;

        if b > 5 {
            break;
        }
    }
}