# Dibagi menjadi 2 subset :
* Scalar: representasi single value yaitu integer, float, boolean, dan char
* Compound: representasi dari multiple value atau beberapa value dalam satu type yaitu tuple dan array

## Scalar
* Integer: tipe data number dalam bilangan bulat
* Float : tipe data number dalam desimal
* Boolean : tipe data yang bernilai true atau false
* Char: tipe data karakter

## Compound
* Tuple: kumpulan beberapa data yang bisa berbeda tipe data
* Array : kumpuluan beberapa data yang harus tipe data yang sama

---
# Explicit Type
saat membuat variable, secara default tidak perlu menyebutkan tipe data secara explicit, karena Rust bisa otomatis mendeteksi tipe data apa yang kita gunakan. Namun bisa juga menyebutkan tipe data sebuah variable secara explicit dengan menggunakan tanda ```:``` titik dua

---
# Number
| Panjang  | Signed | Unsigned |
|----------|--------|----------|
| 8-bit    | i8     | u8       |
| 16-bit   | i16    | u16      |
| 32-bit   | i32    | u32      |
| 64-bit   | i64    | u64      |
| 128-bit  | i128   | u128     |

* Signed: bisa negatif, bisa positif
* Unsigned: hanya bisa positif
* Secara default number bertype ```i32```

## Float
| Panjang  | Float |
|----------|-------|
| 32-bit   | f32   |
| 64-bit   | f64   |

* default: ```f64```

## Usize
tipe data number integer yang panjangnya mengikuti platform sistem operasi misalnya 32bit atau 64bit

| Usize            | Keterangan        |
|------------------|-------------------|
| isize            | 32-bit / 64-bit   |
| usize (unsigned) | 32-bit / 64-bit   |

* isize: Integer size
* usize: Unsigened integer

## Default Number
ketika membuat variable secara implicit(tidak menyebutkan tipe data) maka Rust akan menggunakan default type. Jika bilangan bulat maka ```i32```. Jika bilangan dengan pecahan decimal maka ```f64```.

## Konversi Tipe Data Number
Rust bisa melakukan konversi tipe data dari tipe data Number yang ukuranya kecil ke ukuranya lebih besar begitu juga sebaliknya. Untuk melakukan konversi number pada Rust bisa menggunakan kata kunci ```as``` Namun perlu diperhatikan jika kita lakukan konversi tipe data Number dari ukuran besar -> kecil, maka bisa terjadi yang namanya ```Integer Overflow```.

```Integer Overflow``` adalah kondisi dimana nilai number tidak bisa ditampung oleh tipe data tujuan konversi. Misal kkita punya number ```1000000``` dalam bentu ```i32``` lalu dikonversi ke bentuk ```i8``` maka akan terjadi Integer Overflow. Karena ```i8``` tidak bisa menampung nilai tersebut. 

## Numeric Operations
Rust mendukung semua operasi numerik. Hampir sama dengan kebanyakan bahasa pemrograman lainya

| Operator | Keterangan          |
|----------|---------------------|
| +        | tambah              |
| -        | kurang              |
| /        | bagi                |
| *        | kali                |
| %        | sisa bagi / modulo  |


# Augmented Assignments
Operasi ke variable yang sama

| Numeric Operator | Augmented Assignments |
|------------------|------------------------|
| a = a + 10       | a += 10                |
| a = a - 10       | a -= 10                |
| a = a * 10       | a *= 10                |
| a = a / 10       | a /= 10                |
| a = a % 10       | a %= 10                |

```!Important``` Perlu di ingat karena kita mengubah variable yang sama maka variable yang digunakan harus ```mutable```

--- 

# Boolean

Tipe data yang sederhana, hanya bernilai true dan false. Walaupun sederhana, namun tipe data ini banyak di gunakan di mana-mana, terutama percabangan dan perulangan. 

## Comparison Operators
Operator perbandingan adalah operator yang menghasilkan nilai boolean true atau false

| Comparison Operator | Keterangan               |
|---------------------|--------------------------|
| >                   | Lebih Dari               |
| <                   | Kurang Dari              |
| >=                  | Lebih Dari Sama Dengan   |
| <=                  | Kurang Dari Sama Dengan  |
| ==                  | Sama Dengan              |


## Operasi Boolean

| Operator | Keterangan |
|----------|------------|
| &&       | Dan        |
| \|\|     | Atau       |
| !        | Kebalikan  |

### Operasi &&

| Nilai 1 | Operator | Nilai 2 | Hasil |
|---------|----------|---------|-------|
| true    | &&       | true    | true  |
| true    | &&       | false   | false |
| false   | &&       | true    | false |
| false   | &&       | false   | false |

### Operasi ||

| Nilai 1 | Operator | Nilai 2 | Hasil |
|---------|----------|---------|-------|
| true    | \|\|     | true    | true  |
| true    | \|\|     | false   | true  |
| false   | \|\|     | true    | true  |
| false   | \|\|     | false   | false |

### Operasi !

| Operator | Nilai 2 | Hasil |
|----------|---------|-------|
| !        | true    | false |
| !        | false   | true  |

---
 ## Char
Representasi dari karakter dengan menggunakan petik satu '' pada Rust