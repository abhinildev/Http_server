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

