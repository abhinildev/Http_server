# Http_server
A custom http server using rust 

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


flowchart LR
    A[Main Thread] --> B[ThreadPool::new(4)]
    B --> C1[Worker Thread 1]
    B --> C2[Worker Thread 2]
    B --> C3[Worker Thread 3]
    B --> C4[Worker Thread 4]
    A -->|Incoming Connection| D[Job Queue]
    D -->|Job assigned| C1
    D -->|Next job| C2
    D -->|Next job| C3
    D -->|Next job| C4
    C1 --> E[handle_connection()]
    C2 --> E
    C3 --> E
    C4 --> E
