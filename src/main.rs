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
    let lulus_nilai_akhir = nilai_akhir >= 75;

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

#[test]
fn tuple() {
    /* Harus di sebutkan tipe datanya di dalam (), dan di assign harus sama*/
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);
}

#[test]
fn tuple_access() {
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("{} {} {}", a, b, c);
}

#[test]
fn destructuring_tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    // Destructuring
    // let (a, b, c) = data;
    // println!("{} {} {}", a, b, c);

    let (a, b, _) = data; // C tidak digunakan
    println!("{} {}", a, b);
}

#[test]
fn mutable_tuple() {
    // harus tambahkan mut jika ingin ubah data
    let mut data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    // Destructuring
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 22.5;
    data.2 = false;

    println!("{:?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    // tuple kosong atau unit
    let result = unit();
    println!("{:?}", result);

    // Tuple kosong
    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    /* Array dengan [tipe_data, jumlah_data] artinya fix tidak bisa bertambah atau berkurang panjang array yang dibuat*/
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // let array = [1, 2, 3, 4, 5]; // Bisa langsung
    println!("{:?}", array);
}

#[test]
fn access_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("a: {}, b: {}", a, b);
}

#[test]
fn mutable_array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("a: {}, b: {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);
}

#[test]
fn length_array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("a: {}, b: {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    /*
    Kenapa type panjang array usize ? Karen type panjang array mengikuti type sistem operasi. Jadi maksimal panjang yang bisa di buat array ya tergantung type sistem operasinya 32 atau 64 bit
    */
    let length = array.len();
    println!("{:?}", length);
}

#[test]
fn two_dimensional() {
    let matrix: [[i32; 2]; 2] = [[1, 2], [3, 4]];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
    println!("0 0: {}", matrix[0][0]);
    println!("0 1: {}", matrix[0][1]);
    println!("1 0: {}", matrix[1][0]);
    println!("1 1: {}", matrix[1][1]);
}

// Global variable constant scope
const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    /*
    * Harus disebutkan tipe datanya
    * Harus ada nilainya dan tidak pernah berubah
    * Harus huruf besar
    */
    const MINIMUM: i32 = 0;
    println!("MINIMUM di dalam: {}, MAXIMUM di lura: {}", MINIMUM, MAXIMUM);
}
