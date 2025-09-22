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