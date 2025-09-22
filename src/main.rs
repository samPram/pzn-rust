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
