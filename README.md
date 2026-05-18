# Experiment 2.1: Original code, and how it run

- Server
![server](assets/images/experiment-2.1-server.png)
- Client
![client 1](assets/images/experiment-2.1-client1.png)
![client 2](assets/images/experiment-2.1-client2.png)
![client 3](assets/images/experiment-2.1-client3.png)

Pada eksperimen ini, program dijalankan menggunakan satu server dan tiga client secara bersamaan. Server berfungsi sebagai pusat komunikasi yang menerima pesan dari setiap client lalu melakukan broadcast ke semua client lain yang terhubung. Untuk menjalankan program, pertama menjalankan server menggunakan perintah `cargo run --bin server`, kemudian membuka beberapa terminal lain untuk menjalankan client menggunakan `cargo run --bin client`. Ketika salah satu client mengetik pesan, pesan tersebut dikirim ke server melalui websocket lalu diteruskan kembali ke semua client yang sedang aktif. Hal ini menunjukkan bagaimana asynchronous programming memungkinkan banyak koneksi berjalan secara bersamaan tanpa saling blocking. Selain itu, penggunaan websocket membuat komunikasi berlangsung secara real-time sehingga pesan langsung muncul pada seluruh client yang terhubung.

# Experiment 2.2: Modifying port

Pada eksperimen ini, saya mengubah port websocket dari 2000 menjadi 8080. Perubahan dilakukan pada dua sisi, yaitu file server.rs dan client.rs. Pada sisi server, bagian `TcpListener::bind()` diubah agar server berjalan pada port 8080. Sementara itu, pada sisi client, URI websocket pada `ClientBuilder::from_uri()` juga harus diubah menjadi `ws://127.0.0.1:8080` agar client dapat terhubung ke server yang benar. Jika hanya salah satu sisi yang diubah, maka koneksi websocket akan gagal karena port server dan client tidak sama. Selain itu, protocol websocket yang digunakan pada program ini adalah `ws://`, yang menunjukkan bahwa komunikasi dilakukan menggunakan websocket protocol.

# Experiment 2.3: Small changes, add IP and Port

- Server
![server](assets/images/experiment-2.3-server.png)
- Client
![client 1](assets/images/experiment-2.3-client1.png)
![client 2](assets/images/experiment-2.3-client2.png)
![client 3](assets/images/experiment-2.3-client3.png)

Pada eksperimen ini, saya memodifikasi aplikasi chat dengan menambahkan informasi alamat IP dan port pengirim pada setiap pesan yang dibroadcast oleh server. Perubahan dilakukan pada file `server.rs`, tepatnya pada bagian format pesan sebelum dikirim ke seluruh client. Saya menggunakan nilai `addr` bertipe `SocketAddr` untuk mendapatkan identitas koneksi client yang sedang mengirim pesan. Saya mengubah format broadcast menjadi `IP:PORT: pesan` agar setiap client dapat mengetahui asal pesan yang diterima. Selain itu, saya juga menambahkan `println!("From client {} {:?}", addr, text);` di sisi server sehingga terminal server dapat menampilkan log seperti `From client 127.0.0.1:49838 "hi"` untuk mempermudah monitoring koneksi dan aktivitas client. Pada sisi client, saya menambahkan `println!("Renata's Computer - From server: {}", text);` pada output agar tampilan pesan menjadi lebih jelas dan mudah dibedakan dari terminal lain. Perubahan ini membantu memahami bagaimana websocket server menerima pesan dari satu client lalu meneruskannya kembali ke semua client lain secara asynchronous dan real-time.
