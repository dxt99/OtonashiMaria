# No 8. Low Level Calculator
Direktori ini berisi implementasi kalkulator sederhana menggunakan bitwise operator dalam C++.

## Cara Menjalankan
Pada direktori ini, jalankan `make build` untuk membuat executable, dan `make run` untuk menjalankannya.

## Spesifikasi Kalkulator
Kalkulator memiliki dua mode, yaitu evaluasi ekspresi aritmetika dan evaluasi nilai inverse square root. 

- Ekspresi Aritmetika
Evaluasi ekspresi aritmetika dapat menerima satu baris ekspresi dengan operator tambah(+), kurang(-), kali(\*), bagi(/), pangkat(^). Operand adalah signed integer 32-bit, dan semua operasi juga memiliki hasil signed integer 32-bit. Operator dan operand __harus dipisah oleh spasi__, dan akan dievaluasi dari kiri ke kanan. Berikut contoh ekspresi yang benar:
`
1 + 2 * 3 / 2 - -5 ^ 2
`

- Inverse Square Root
Evaluasi nilai inverse square root dapat menerima satu bilangan floating point, dan akan mengeluarkan bilangan floating point hasil inverse square root dari masukan. Evaluasi ini dipisah dengan evaluasi ekspresi aritmetika integer karena hasil dari inverse square root jika dikerjakan secara integer (dengan masukan dan keluaran integer) hampir selalu nol.

## Spesifikasi Source Code
Kode kalkulator terbagi menjadi dua bagian. File `main.cpp` berisi user interface, sanitasi input, dan pemanggilan fungsi kalkulasi, dan file `op.cpp` berisi fungsi-fungsi kalkulasi yang dapat digunakan user. File `op.cpp` berisi implementasi bitwise dari penjumlahan, pengurangan, absolute value, perkalian, pembagian, pangkat, dan inverse square root.