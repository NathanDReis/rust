pub fn imutaveis() {
    let x: i32 = 5;
    // x = 10;
    println!("x => {}", x);
}

pub fn mutaveis() {
    let mut x = 10;
    let mut y = x;
    println!("x, y => {} {}", x, y);

    x = 15;
    y += 2;
    println!("x, y => {} {}", x, y);
}

pub fn constantes() {
    const z: i32 = 20;
    println!("O valor de Z é: {}", z);
}

pub fn shadowing() {
    let a: i32 = 25;
    println!("O valor de 'a' é: {}", a);

    let a: &str = "Texto";
    println!("O valor de 'a' é: {}", a);

    let a: [u32; 5] = [1, 2, 3, 4, 5];
    println!("O valor de 'a' é: {:?}", a);
}