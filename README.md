# 🦀 HTTP Server in Rust

A lightweight multithreaded HTTP server built in **Rust** from scratch.

It listens for incoming connections, handles multiple requests concurrently using a **ThreadPool**, and returns a simple HTML response.  
This project is designed to understand the fundamentals of **HTTP servers**, **networking**, and **concurrency** in Rust.

---

## ⚙️ Features

- Basic HTTP request parsing  
- Multi-threaded connection handling using `ThreadPool`  
- Returns `200 OK` for `/` and `404 Not Found` for other routes  
- Clean shutdown behavior  

---

## 🧠 Architecture Diagrams

### 🔹 HTTP Request Flow

```mermaid
flowchart TD
    A[Client Browser] -->|Sends HTTP Request| B[TcpListener on 127.0.0.1:7878]
    B -->|Accepts incoming connection| C[ThreadPool]
    C -->|Assigns worker thread| D[handle_connection()]
    D --> E{Route check}
    E -->|"/"| F[Return 200 OK + "Hello, World"]
    E -->|Other routes| G[Return 404 Not Found]
    F --> H[Write response to stream]
    G --> H
    H --> I[Flush stream and close connection]
