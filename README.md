# 🦀 rust-http-server

A concurrent HTTP/1.1 server built from scratch in Rust — no frameworks, no dependencies, standard library only.

---

## 🚀 How to Run

### Prerequisites
- Rust installed ([rustup.rs](https://rustup.rs))

### Run

```bash
git clone https://github.com/odedabas123/rust-http-server.git
cd rust-http-server
cargo run
```

Then open `http://127.0.0.1:8080` in your browser.

---

## 🏗️ How It Works

```
Browser → TCP connection → TcpListener
                               ↓
                         channel (mpsc)
                               ↓
                    Thread Pool (4 workers)
                               ↓
                      handle_connection()
                               ↓
                parse request → router → read file
                               ↓
                       HTTP response → browser
```

### Request Lifecycle
1. `TcpListener` accepts a connection on port 8080
2. Connection is sent to a worker thread via `mpsc` channel
3. Worker parses the HTTP request line (`GET / HTTP/1.1`)
4. Router matches the path to a file in `/public`
5. File is read with `fs::read_to_string` and sent back as HTTP response
6. Response time is logged to the terminal

---

## ⚙️ Features

- **TCP socket server** — raw socket programming with `std::net`
- **HTTP/1.1 parsing** — reads and parses request lines and headers
- **Static file serving** — serves HTML files from the `/public` directory
- **Thread pool** — 4 worker threads handle requests concurrently using `Arc<Mutex<>>` and `mpsc` channels
- **Router** — maps URL paths to files, returns 404 for unknown routes
- **Request logging** — logs method, path, status and response time for every request

---

## 📁 Project Structure

```
rust-http-server/
├── src/
│   └── main.rs        # All server logic
├── public/
│   ├── index.html     # Home page
│   ├── about.html     # About page
│   └── 404.html       # 404 page
├── Cargo.toml
└── README.md
```

---

## 🧠 Rust Concepts Used

| Concept | Where |
|---|---|
| `TcpListener` / `TcpStream` | Accepting and handling connections |
| `BufReader` | Reading HTTP request line by line |
| `fs::read_to_string` | Reading files from disk |
| `thread::spawn` | Creating worker threads |
| `mpsc::channel` | Sending jobs from main to workers |
| `Arc<Mutex<>>` | Sharing the receiver safely across threads |
| `Box<dyn FnOnce()>` | Storing closures as jobs |
| `match` | HTTP router and error handling |

---

## 📊 Example Logs

```
Server running on http://127.0.0.1:8080
Worker 2 got a job; executing.
[GET / HTTP/1.1] HTTP/1.1 200 OK 2ms
Worker 0 got a job; executing.
[GET /about.html HTTP/1.1] HTTP/1.1 200 OK 1ms
Worker 3 got a job; executing.
[GET /fake HTTP/1.1] HTTP/1.1 200 OK 1ms
```

---

## 👨‍💻 Author

Built by Oded Abas — Bar-Ilan University, Computer Science
