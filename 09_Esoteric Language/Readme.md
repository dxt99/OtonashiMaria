# No 9. Esoteric Language: 135
Direktori ini berisi source code interpreter bahasa 135, dokumentasi bahasa 135, dan beberapa sample code dalam bahasa 135.

## Cara Menjalankan

Pada direktori ini, jalankan `make build` untuk membuat executable interpreter. Kemudian, jalankan executable dengan memberi parameter yaitu file source code `.135`. Pada direktori ini, sudah tersedia beberapa sample code dalam direktori test. Berikut contoh cara menjalankan interpreter dengan sample code dari command line:

```
$./135.exe test/HI.135
```

## Dokumentasi Bahasa 135
Bahasa 135 terinsipirasi oleh Brainfuck dan memiliki mekanisme yang serupa. Bahasa ini didesain untuk melatih programmer untuk merasa takut saat mendengar atau melihat angka 135. Program yang ditulis dalam bahasa 135 harus memiliki ekstensi file `.135` agar dapat jalankan oleh interpreter 135.

- ### Operator
Programmer diberikan array of unsigned character berukuran 135, dimana semua entri array diinisialisasi dengan nol. Pointer array dimulai pada elemen pertama (ber-indeks 1), dan array bersifat siklik. Pointer dan elemen dapat dimanipulasi dengan perintah berikut:

```
Semua mode
**: mengubah mode dari mode elemen menjadi mode pointer atau sebaliknya (default mode = elemen)
& : mengoutput elemen yang sedang ditunjuk (sebagai character)
| : menerima masukan string dan memasukkannya pada elemen yang sedang ditunjuk
% : mengeksekusi perintah atau blok selanjutnya apabila elemen yang ditunjuk bernilai 135
^ : membentuk blok (dijelaskan lebih lanjut dibawah)
Mode elemen
+ : menambah value elemen yang ditunjuk sebanyak satu
- : mengurangi value elemen yang ditunjuk sebanyak satu
* : value elemen yang ditunjuk dikali dua
/ : value elemen yang ditunjuk dibagi dua (floor division)
Mode pointer 
+ : menambah value indeks pointer sebanyak satu
- : mengurangi value indeks pointer sebanyak satu
* : value indeks pointer dikali dua
/ : value indeks pointer dibagi dua (floor division)
```

Blok kode dapat dibentuk menggunakan dua buah operator ^. Seluruh operator yang diapit oleh kedua operator ^ akan dieksekusi secara berurut dan berulang sampai elemen yang berada pada indeks 135 bernilai nol. Satu blok tidak harus berada dalam baris yang sama.

- ### Ekspresi
Tiap baris dari program 135 harus merupakan ekspresi matematika yang valid yang hanya menggunakan digit 1, 3, dan 5, serta operasi dengan rincian sebagai berikut:

```
**: pangkat
& : bitwise and
| : bitwise or
% : modulo
^ : bitwise xor
+ : penjumlahan
- : pengurangan
* : perkalian
/ : pembagian
```

Ekspresi akan dievaluasi dari kiri ke kanan (1 + 2 * 3 = 9), dan hasil evaluasi ekspresi tiap baris yang tidak kosong harus bernilai 135. Karakter spasi, carriage return, dan tab, akan diabaikan. Semua karakter lainnya yang tidak disebutkan adalah karakter illegal dan akan menyebabkan syntax error.

- ### Contoh program singkat
Program dibawah akan mengoutput "HI" pada layar.
```
3 ** 3 - 1 ** 1 + 1 * 5 ** 1 
31 + 3 ** 1 + 11 * 1 * 3 * 1
13 + 13 * 5  ^ 5135 * 1 ^ 5135 & 135 + 5 & 135
```

Program dibawah ini menerima masukan user dan mencetak "><" jika karakter pertama dari input adalah "@"
```
135 ** 1 - 3 ** 1 + 3
1 ** 1 + 1 ** 1 | 135
15 + 13 + 1 + 15 * 3 + 3
135 % 5 ^ 135
135 / 1 - 1 - 1 - 1 - 1 - 1 & 135 + 5
51 - 13 - 31 - 5 & 1 + 135
135 ** 1 - 3 ** 1 - 5 ** 1 + 5 + 3 % 15 ^ 135
```