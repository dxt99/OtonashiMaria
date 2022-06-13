# No 8. Low Level Calculator
Direktori ini berisi implementasi kalkulator menggunakan bitwise operator dalam C++.

## Cara Menjalankan
Pada direktori ini, jalankan `make build` untuk membuat executable, dan `make run` untuk menjalankannya.

## Spesifikasi Kalkulator
Kalkulator memiliki dua mode, yaitu evaluasi ekspresi aritmetika dan evaluasi nilai inverse square root. 

### Ekspresi Aritmetika
Evaluasi ekspresi aritmetika dapat menerima satu baris ekspresi dengan operator tambah(+), kurang(-), kali(*), bagi(/), pangkat(^). Operand adalah signed integer 32-bit, dan semua operasi juga memiliki hasil signed integer 32-bit. Operator dan operand harus dipisah oleh spasi, dan akan dievaluasi dari kiri ke kanan. Contoh ekspresi yang benar:
'''
1 + 2 * 3 / 2 - -5 ^ 2
'''

### Evaluasi Inverse Square Root
Evaluasi nilai inverse square root dapat menerima satu bilangan floating point, dan akan mengeluarkan bilangan floating point hasil inverse square root dari masukan. Evaluasi ini dipisah dengan evaluasi ekspresi aritmetika integer karena hasil dari inverse square root jika dikerjakan secara integer (dengan masukan dan keluaran integer) hampir selalu nol.