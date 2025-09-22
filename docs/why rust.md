# Why Rust ?
* Keamanan memori, sistem pemrosesan memori yang aman. Menutup kekurangan C/C++
* Kinerja tinggi. Dirangcang memiliki keinerja yang sangat baik bahkan mendekati C/C++
* Concurrency yang aman. Punya concurrency bawaan yang aman bermanfaat ketika butuh aplikasi dengan trafic tinggi 
* Pemeliharaan kode yang baik. Dari awal Rust mendukung praktik pengkodean yang bersih dan aman oleh karena itu programmer akan di paksa untuk membuat kode yang selalu baik

---
# Ekosistem
```mermaid
flowchart TD
    A[Rust Compiler] -->|Compile| B[Kode Rust (File.rs)]
    B --> C[Rust Binary (File / File.exe)]
    A -->|Binary| C
```
----
# Command
* Cek rust yang terinstall : ```$ rustup check```
* Update rust: ```$ rustup update```
* Cek version: ```$ rustc --version```
* Cek `cargo` version: ```$ cargo --version````
* Buat project baru `cargo`: ```$ cargo new {name-project}```

---
# Structure Directory
* src: main project koder rust
* Cargo: file config for cargo
* target: hasil kompilasi program. Bisa akses binary langsung `./target/debug/{name}`
* release: hasil kompilasi akhir program. Bisa akses binary file `./target/release/{name}`

---
# Function Bawaan
Naming function pada Rust dianjurkan menggunakan huruf kecil semua, kalau tidak akan dikomplain tidak sesuai praktik yang baik menurut Rust-nya.

## Main function
Gerbang utama menjalankan kode rust. Sama seperti C/C++, Java atau Golang. Yang membedakan kata kuncinya menggunakan ```fn```. Dalam Rust hanya bisa menggunakan satu main function.

## Print Function
1. print! 
```rust
print!(text);
```
Untuk menulis
2. println!(text)
```rust
println!();
```
Untuk menulis dan diakhiri dengan ENTER

text / string menggunakan ""

---

# Cargo
Cargo adalah Package Manager bawaan untuk Rust. Dapat melakukan proses kompilasi, dependency management, testing, dan lain lain secara mudah. 

## Command cargo
* menjalankan kode rust : `cargo run`. Dia akan build, di compile menjadi binary file(debug), binary file di jalankan.
* Build distribution file: `cargo build --release`
* Compile ke binary file: `cargo build`

## Distribution file
File hasil akhir dari projek yang di buat. Menggunakan `cargo build --release`

---
# Unit Test
Kode yang dikhusukan untuk percobaan atau pengujian. Untuk membuat unit test di Rust bisa membuat function dengan nama apapun lalu ditambahkan Attribute / Annotation test pada function tersebut. Cara running test `cargo test {name_test_function} -- --exact`