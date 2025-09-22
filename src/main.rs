fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, world!");
}

#[test]
fn test_variable() {
    let name = "Aldinsyah Dzikri Pramadafi";

    // name = "Teko"; // Cannot assign value to immutable
    println!("Name: {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Aldinsyah Dzikri Pramadafi";
    println!("Name: {}", name);

    name = "Pramadafi";
    println!("Name: {}", name);
}

#[test]
fn static_typing() {
    let mut name = "Aldinsyah Dzikri Pramadafi"; // warning karena tidak perlu mut karena variable tidak pernah di ubah di setelah kode variable terbuat
    println!("Name: {}", name);

    // name = 10; // expected &str tapi di isi i32
    println!("Name: {}", name);
}

#[test]
fn shadowing() {
    let name = "Aldinsyah Dzikri Pramadafi";
    println!("Name: {}", name);

    let name = 10; // disimpan di lokasi memory yang berbeda
    // mengacu ke variable name yang baru
    println!("Name: {}", name); // jika di print maka akan tertutup variable yang baru
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("Age: {}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("a: {}", a);

    let b: f32 = 10.5;
    println!("b: {}", b);
}

#[test]
fn number_convertion() {
    let a: i8 = 10;
    println!("a: {}", a);

    let b: i16 = a as i16;
    println!("b: {}", b);

    let c: i32 = b as i32;
    println!("c: {}", c);

    let d: i64 = 1000000000;
    // Integer Overflow: Nilai tidak bisa di tampung di variable 'e'
    let e: i8 = d as i8; // tidak ada error pada code secara default
    println!("e: {}", e);
}


#[test]
fn numeric_operations() {
    let a = 10;
    let b = 10;

    let c = a * b;
    println!("c: {}", c);
    let d = a / b;
    println!("d: {}", d);
    let e = a + b;
    println!("e: {}", e);
}

#[test]
fn augmented_assignment() {
    // let a = 10; // tidak bisa, karena variable harus mutable
    let mut a = 10;
    println!("a: {}", a);

    a += 10;
    println!("a: {}", a);

    a -= 10;
    println!("a: {}", a);
}


#[test]
fn boolean() {
    let a = true;

    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 20;
    let b = 20;

    let result: bool = a > b;
    // let result: bool = a >= b; // bisa, tapi kena shadowing

    println!("result: {}", result);
}

#[test]
fn boolean_operator() {
    // let absen = 70;
    let absen = 75;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >=75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("{}", lulus_final); // Cok ora lulus cok wkwk
}

#[test]
fn char_type() {
    let char1: char = 'a';
    // let char1: char = 'abc'; // to many character on char literal
    let char2: char = 'b';

    println!("{} {}", char1, char2);
}