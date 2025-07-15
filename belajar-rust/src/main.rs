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
    let mut tup1: (i32, f64, char) = (42, 6.28, 'Z'); 
    let mut arr1: [i32; 3] = [1, 2, 3];               

    println!("Tuple: {:?}", tup1);
    println!("Array: {:?}", arr1);

    tup1.0 = 95;
    tup1.1 = 3.14;
    tup1.2 = 'A';
    println!("\nUpdated Tuple: {:?}", tup1);
    
    arr1[0] = 10;
    arr1[1] = 20;
    arr1[2] = 30;
    println!("Updated Array: {:?}", arr1);
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

#[test]
fn augmented_assignment() {
    let mut a: i32 = 10;
    println!("\nData awal adalah {}", a);

    a += 5;
    println!("Hasil a += 5 adalah {}", a);

    a -= 3;
    println!("Hasil a -= 3 adalah {}", a);
}

#[test]
fn boolean() {
    let a: bool = true;
    let b: bool = false;
    println!("a = {}, b = {}", a, b);
}

#[test]
fn comparison() {
    let a: i32 = 25;
    let b: i32 = 25;
    println!("a = {}, b = {}", a, b);

    let result: bool = b >= a;
    println!("Hasil dari b > a adalah {}", result);
}

#[test]
fn boolean_operator() {
    let absen:i32 = 70;
    let nilai:i32 = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai: bool = nilai >= 75;

    let lulus: bool = lulus_absen && lulus_nilai;
    println!("Apakah lulus ? [{}]", lulus);
}

#[test]
fn unit() {
    println!("Hello, world")
}

#[test]
fn test_unit() {
    let hasil = unit();
    println!("hasil: {:?}", hasil);
}

#[test]
fn tuple() {
    let mut data: (bool, char, f64) = (false, 'Z', 13.50);
    println!("\nData: {:?}", data);

    let (a, b, c) = data;
    println!("\na = {}, b = {}, c = {}", a, b, c);

    data.0 = true;
    data.1 = 'A';
    data.2 = 3.14;
    println!("\nData: {:?}", data);

    let (a, b, c) = data;
    println!("\na = {}, b = {}, c = {}", a, b, c);
}

#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nArray: {:?}", array);
    
    let [a, b, c, d, e] = array;
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    array[0] = 10;
    array[1] = 20;
    array[2] = 30;
    array[3] = 40;
    array[4] = 50;
    println!("\nArray: {:?}", array);

    let [a, b, c, d, e] = array;
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    let leght = array.len();
    println!("\nPanjang array adalah {}", leght);
}

#[test]
fn two_dimesion_array() {
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("\nMatrix: {:?}", matrix);

    println!("\nKordinat matrix (1): {:?}", matrix[0]);
    println!("Kordinat matrix (2): {:?}", matrix[1]);
    println!("Kordinat matrix (3): {:?}", matrix[2]);

    println!("\nKordinat (0, 0): {:?}", matrix[0][0]);
    println!("Kordinat (0, 1): {:?}", matrix[0][1]);
    println!("Kordinat (0, 2): {:?}", matrix[0][2]);

    println!("\nKordinat (1, 0): {:?}", matrix[1][0]);
    println!("Kordinat (1, 1): {:?}", matrix[1][1]);
    println!("Kordinat (1, 2): {:?}", matrix[1][2]);

    println!("\nKordinat (2, 0): {:?}", matrix[2][0]);
    println!("Kordinat (2, 1): {:?}", matrix[2][1]);
    println!("Kordinat (2, 2): {:?}", matrix[2][2]);
}