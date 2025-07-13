fn main() {
    println!("Hello, budi");

    println!("Hello, riski");
    
    println!("Hello, budi riski");
}

#[test]
fn hello_test() {
    println!("This is a test function");
}

#[test]
fn test_variabel() {
    let name: &str = "putu riski wijaya";
    println!("Hello, {}", name);
}

#[test]
fn static_typing() {
    let mut name: &str = "putu riski wijaya";
    println!("Hello, {}", name);
    
    name = "Timoty Ronald";
    println!("Hello, {}", name);
    
}

#[test]
fn shadowing_typing() {
    let name: &str = "putu riski wijaya";
    println!("Hello, {}", name);
    
    let name: i64 = 123;
    println!("Hello, {}", name);
    
}

#[test]
fn scalar_types() {
    let mut x: i32 = 10;         
    let mut y: f64 = 3.14;       
    let mut is_active: bool = true; 
    let mut letter: char = 'A';  

    println!("x = {}, y = {}, is_active = {}, letter = {}", x, y, is_active, letter);

    x = 50;         
    y = 10.79;       
    is_active = false; 
    letter = 'E';

    println!("x = {}, y = {}, is_active = {}, letter = {}", x, y, is_active, letter);
}

#[test]
fn number_conversion() {
    let a: i8 = 100;
    println!("a = {}", a);
    
    let b: i16 = a as i16; 
    println!("b = {}", b);
    
    let c: i32 = b as i32; 
    println!("c = {}", c);
    
    let d: i64 = c as i64; 
    println!("d = {}", d);
}

#[test]
fn compound_types() {
    let tup: (i32, f64, char) = (42, 6.28, 'Z'); 
    let arr: [i32; 3] = [1, 2, 3];               

    println!("Tuple: ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("Array: [{}, {}, {}]", arr[0], arr[1], arr[2]);
}

#[test]
fn numeric_operator() {
    let a: i32 = 10;
    let b: i32 =5;

    let tambah: i32 = a + b;
    println!("Hasil dari {} tambah {} adalah {}", a, b, tambah);

    let kurang: i32 = a - b;
    println!("Hasil dari {} kurang {} adalah {}", a, b, kurang);

    let kali: i32 = a * b;
    println!("Hasil dari {} kali {} adalah {}", a, b, kali);

    let bagi: i32 = a / b;
    println!("Hasil dari {} bagi {} adalah {}", a, b, bagi);
}