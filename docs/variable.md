# Variable
Tempat untuk menyimpan data. Menggunakan kata kunci ```let```. Setelah variable diisi data, maka tidak bisa diubah lagi datanya atau imutable

## Mutable
Rust memperbolehkan jika kita ingin membuat variable yang bisa di ubah lagi atau Mutable. Caranya dengan menambahkan kata kunci ```let mut``` ketika membuat variable

---

## Static Typing
Rust adalah bahasa yang static typeing, artinya setiap kita membuat variable dengan jenis data tertentu, maka dia tidak akan bisa berubah menjadi tipe data lainya. Misalnya sudah punya variable test/string, maka tidak bisa mengubahnya ke data angka/number

---

## Shadowing
Di rust bisa membuat variable dengan nama yang sama. Namun saat kita membuat variable dengan nama yang sama maka variable sebelumnya akan tertutup atau disebut ```shadowing```. Praktik ini mungkin kurang baik jika dilakukan terlalu sering, karena bisa membingungkan yang membaca kode kita, namun ini di perbolehkan di Rust

---

## Variable Scope
Seperti pada bahasa pemrograman lain pembuatan variable hanya bisa diakses di dalam scope ```{..}``` atau di dalam scope lain. Jika dari dalam scope ke luar itu tidak bisa
