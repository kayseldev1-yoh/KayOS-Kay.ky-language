# Rencana Aksi Terpadu: Pengembangan Toolset & Inisialisasi KayOS

## 1. Pengembangan Tool Keamanan (Toolkit KayOS)
- **Net-Map (Scanner):** Implementasi pemindaian menggunakan *raw socket* (memanggil `socket`, `connect` via FFI `libc`).
- **Enc-Kit (Enkripsi):** Implementasi fungsi enkripsi dasar menggunakan FFI `openssl` atau pustaka enkripsi sistem.

## 2. Inisialisasi KayOS (boot.ky)
- **Boot Sequence:** Membuat skrip `boot.ky` yang dijalankan oleh shell saat masuk (seperti `.bashrc` atau `/etc/rc.local`).
- **Fungsi Boot:**
  - Inisialisasi *network interface*.
  - Menjalankan *background service* (seperti *logging* atau *security monitor*).
  - Menampilkan *banner* selamat datang KayOS.

## 3. Implementasi Detail
1. **Net-Map.ky:** Menulis skrip `net_map.ky` di direktori `examples/` untuk memicu pemanggilan FFI ke `libc`.
2. **Boot.ky:** Menulis skrip inisialisasi yang akan dibaca oleh `run_shell()`.
3. **Penyempurnaan Shell:** Menambahkan perintah `boot` dan `scan` ke dalam `KayOS Shell` agar dapat mengeksekusi file `.ky` tersebut.

## 4. Validasi
- Memastikan `boot.ky` terbaca dengan benar oleh VM saat shell dimulai.
- Memastikan `net_map` dapat memanggil fungsi `libc` dengan argumen yang tepat tanpa *crash*.
