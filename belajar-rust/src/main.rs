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
    let mut name: &str  = "putu riski wijaya";
    println!("Hello, {}", name);
    
    name = "Timoty Ronald";
    println!("Hello, {}", name);
    
}