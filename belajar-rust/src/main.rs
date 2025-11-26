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
    let a: i32 = 30;
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
    let hasil: () = unit();
    println!("hasil: {:?}", hasil);
}

#[test]
fn tuple() {
    let mut data: (bool, char, f64) = (false, 'Z', 13.50);
    println!("\nData: {:?}", data);

    let (a, b, c) = data;
    println!("a = {}, b = {}, c = {}", a, b, c);

    data.0 = true;
    data.1 = 'A';
    data.2 = 3.14;
    println!("\nData: {:?}", data);

    let (a, b, c) = data;
    println!("a = {}, b = {}, c = {}", a, b, c);
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

#[allow(dead_code)]
const WAKTU_TUNGGU_MAKSIMUM: u32 = 60;
#[allow(dead_code)]
const PI: f64 = 3.14159;

#[test]
fn test_const() {
    println!("WAKTU_TUNGGU_MAKSIMUM = {}", WAKTU_TUNGGU_MAKSIMUM);
    println!("PI = {}", PI);
}
#[test]
fn variabel_scope() {
    let eko: i32 = 1;

    {
        let budi: i32 = 2;
        println!("\neko = {}, budi = {}", eko, budi);
    }

    {
        let riski: i32 = 3;
        println!("eko = {}, riski = {}", eko, riski);
    }

    println!("\neko = {}", eko);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

#[test]
fn function_a() {
    let a: i32 =10;
    let b: String = String::from("eko");

    println!("\n{}, {}", a, b);
}

#[test]
fn function_b() {
    let a: i32 = 15;
    let b: String = String::from("budi");

    println!("{}, {}", a, b);
}

#[test]
fn string() {
    let name: &str = "  Eko arab kurniawan  ";
    let trim: &str = name.trim();
    
    println!("\nNama awal: [{}]", name);
    println!("Data ahkir: [{}]", trim);

    let mut username: &str = "Budi hartono";
    println!("\nUsername awal: {}", username);
    
    username = "Timoty Ronald";
    println!("Username akhir: {}", username);
}

#[test]
fn string_type() {
    let mut name0: String = String::from("Eko Kurniawan");
    println!("\nNama: {}", name0);
    
    name0.push_str(" khanedy");
    println!("\nNama: {}", name0);

    let name1 = name0.replace("Eko", "Budi");
    println!("\nNama: {}", name1);
}

#[test]
fn ownership_rules() {
    // Data a belum bisa di akses disini, belum di deklarasikan
    let a =10; // Data a bisa di akses di sisni

    { // Data b belum bisa di akses disini, belum di deklarasikan
        let b = 20; // Data b bisa di akses di sisni
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak dapat di akses lagi

    println!("{}", a);
}   // scope a selesai, a dihapus, a tidak dapat di akses lagi

#[test]
fn ownership_movement() {
    let name1 = String::from("Timoty Ronald");
    println!("[1] {}", name1);

    // Ownership berpindah dari name1 ke name2
    let name2 = name1; 
    // name1 tidak dapat diakses lagi setelah ini
    
    println!("[2] {}", name2); // name2 sekarang memiliki ownership dari String
}

#[test]
fn clone() {
    let name1 = String::from("Timoty Ronald");
    println!("[1] {}", name1);

    // Clone membuat salinan dari String
    let name2 = name1.clone(); 
    // name1 masih dapat diakses setelah ini
    
    println!("[2] {}, {}", name1, name2) // name1 masih memiliki ownership dari String
    // name2 sekarang memiliki salinan dari String
}

#[test]
fn if_expresion() {
    let value: i32 = 9;

    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("[{}]", result);
}

#[test]
fn loop_expreseion() {
    let mut counter  = 0;
    loop {
        counter += 1;

        if counter == 100 {
            break;
        } else if counter % 2 == 1 {
            continue;
        }

        println!("counter = {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; 
        }
    };
    println!("result = {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer; 
            }
            
            println!("\n[{}] x [{}] = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break; 
            }
        }
        number += 1;
    } 
    println!("\nLoop finished");
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("counter = {}", counter);
        } 
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let arr: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < arr.len() {
        println!("Value {}: {}", index, arr[index]);
        index += 1;
    }      
    
    println!("");

    let mut iter = arr.iter().enumerate();
    while let Some((index, value)) = iter.next() {
        println!("(while let) Value {}: {}", index, value);
    }
}

#[test]
fn array_iteration_for_loop() {
    let arr: [&str; 5] = ["Z", "Y", "X", "W", "P"];

    for value in arr {
        println!("Value: {}", value);
    }     

    println!("");

    for (index, value) in arr.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }  

    print!("\n");

    for index in 0..arr.len() {
        println!("Index: {}, Value: {}", index, arr[index]);
    }

    println!("");

    for index in (0..arr.len()).rev() {
        println!("(reverse) Index: {}, Value: {}", index, arr[index]);
    }

    println!("");

    for value in arr.iter() {
        println!("(iter) Value: {}", value);
    }
}

#[test]
fn range() {
    let arr: [&str; 5] = ["Z", "Y", "X", "W", "P"];

    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End  : {}", range.end);
    
    println!("");

    for index in 0..=4 {
        println!("Index: {}, Value: {}", index, arr[index]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;

    println!("Start: {:?}", range.start());
    println!("End  : {:?}", range.end());

    let arr: [&str; 5] = ["Z", "Y", "X", "W", "P"];
    
    println!("");

    for index in range {
        println!("Index: {}, Value: {}", index, arr[index]);
    }
}
#[allow(dead_code)]
fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye, {} {}", first_name, last_name);
}

#[test]
fn goodbye_fn() {
    println!("");
    say_goodbye("Putu", "Chandra");
    say_goodbye("Budi", "Hartono");
    say_goodbye("Timoty", "Ronald");
    say_goodbye("Eko", "Kurniawan");
    say_goodbye("Riski", "Wijaya");
}

#[allow(dead_code)]
fn factorial_loop(n: i32) -> i32 {
    let mut result = 1;
    let mut i = 1;

    while i <= n {
        result *= i;
        i += 1;
    }

    result
    
}

#[test]
fn tetst_factorial_loop() {
    let result = factorial_loop(3);
    println!("Factorial of 5 is: {}", result);

    let result = factorial_loop(0);
    println!("Factorial of 5 is: {}", result);
}