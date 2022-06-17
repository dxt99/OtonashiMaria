# No 11. Version Control System: geet
Direktori ini berisi implementasi version control system sederhana bernama geet.

## Instalasi Program
Program geet dibuat menggunakan bahasa [Rust](https://www.rust-lang.org/tools/install) dan memerlukan compiler Rust untuk instalasi. Program dapat dibangun dengan menjalankan `cargo build --release`, dan executable akan tersedia pada `./target/release/geet.exe`. Untuk mempermudah eksekusi geet, tambahkan executable geet pada [Enviroment Path](https://docs.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)).

## Spesifikasi Geet
Command geet dapat dijalankan dengan `geet.exe {command}` atau `geet {command}` jika executable sudah ditambahkan pada enviroment path. Berikut daftar command yang tersedia pada geet:

- init: Menginisialisasi current working directory sebagai repository geet. Contoh: `geet init`
- add: Menambah semua perubahan yang ada di repository pada staging area . Contoh: `geet add`
- remove: Membuang semua perubahan yang sudah ditambahkan ke staging area. Contoh: `geet remove`
- commit {message}: Mengcommit perubahan yang sudah distage. Commit message bersifat opsional. Contoh: `geet commit "hi"`
- log: Melihat daftar commit yang pernah dilakukan. Contoh: `geet log`
- head {version-id}: Merubah isi repositori menjadi versi lain yang pernah dicommit. Contoh: `geet head 10`