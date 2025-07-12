use std::io::{self, Write};

fn main() {
    loop {
        println!("\n==============================");
        println!("      KALKULATOR STANDAR      ");
        println!("==============================");
        println!("Pilih operasi:");
        println!("  1. Tambah (+)");
        println!("  2. Kurang (-)");
        println!("  3. Kali   (*)");
        println!("  4. Bagi   (/)");
        println!("  5. Keluar");
        print!("Masukkan pilihan [1-5]: ");
        io::stdout().flush().unwrap();

        let operasi = read_input();
        let operasi: u32 = match operasi.trim().parse() {
            Ok(num) if num >= 1 && num <= 5 => num,
            _ => {
                println!("Pilihan tidak valid. Silakan coba lagi.");
                continue;
            }
        };

        if operasi == 5 {
            println!("Terima kasih telah menggunakan kalkulator!");
            break;
        }

        let angka1 = match read_number("Masukkan angka pertama: ") {
            Some(num) => num,
            None => continue,
        };
        let angka2 = match read_number("Masukkan angka kedua: ") {
            Some(num) => num,
            None => continue,
        };

        let hasil = match operasi {
            1 => angka1 + angka2,
            2 => angka1 - angka2,
            3 => angka1 * angka2,
            4 => {
                if angka2 == 0.0 {
                    println!("Error: Tidak bisa membagi dengan nol!");
                    continue;
                } else {
                    angka1 / angka2
                }
            },
            _ => unreachable!(),
        };
        println!("\nHasil: {}", hasil);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    input
}

fn read_number(prompt: &str) -> Option<f64> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let input = read_input();
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Input angka tidak valid. Silakan coba lagi.");
            None
        }
    }
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