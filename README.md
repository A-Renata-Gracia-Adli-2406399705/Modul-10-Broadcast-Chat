# Experiment 2.1: Original code, and how it run

- Server
![server](assets/images/experiment-2.1-server.png)

- Client
![client 1](assets/images/experiment-2.1-client1.png)
![client 2](assets/images/experiment-2.1-client2.png)
![client 3](assets/images/experiment-2.1-client3.png)

Pada eksperimen ini, program dijalankan menggunakan satu server dan tiga client secara bersamaan. Server berfungsi sebagai pusat komunikasi yang menerima pesan dari setiap client lalu melakukan broadcast ke semua client lain yang terhubung. Untuk menjalankan program, pertama menjalankan server menggunakan perintah `cargo run --bin server`, kemudian membuka beberapa terminal lain untuk menjalankan client menggunakan `cargo run --bin client`. Ketika salah satu client mengetik pesan, pesan tersebut dikirim ke server melalui websocket lalu diteruskan kembali ke semua client yang sedang aktif. Hal ini menunjukkan bagaimana asynchronous programming memungkinkan banyak koneksi berjalan secara bersamaan tanpa saling blocking. Selain itu, penggunaan websocket membuat komunikasi berlangsung secara real-time sehingga pesan langsung muncul pada seluruh client yang terhubung.