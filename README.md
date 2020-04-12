# In-memory DB performance comparison between python, Go, and Rust

This repository contains codes for blog [post](https://tech-blog.abeja.asia/entry/2020/04/09/115152).

## Usage

### Run server
Run either a python, Go, and Rust server implementation.
You can see the launch commands for each language in the following.

#### python
```bash
$ cd pyhton
python$ python main.py
start server.
```

#### Go
```bash
$ cd go
go$ go run main.go
```

#### Rust
```bash
# Running the server with 16 core threads and a DB implementation
# called `std` which employs `std::sync::RwLock` for locking the DB.
$ cd rust
rust$ cargo run --release -- -p 16 std
    Finished release [optimized] target(s) in 0.05s
     Running `target/release/rust`
Created the async runtime with 16 core threads.
Listening to [::]:50051 using StdDb (with std::sync::RwLock).
```

For more information about the command line arguments, see `cargo run --release -- --help`.

Each DB implementation employs different `RwLock`.
Currently the following DB implementations are available:

| DB impl    | RwLock                    | Async aware lock? | Locking policy          |
|:-----------|:------------------------- |:------------------|:----------------------- |
| `std`      | `std::sync::RwLock`       | no                | OS dependent. On Linux, it will be read-preferring. |
| `asyncstd` | `async_std::sync::RwLock` | yes               | read-preferring         |
| `tokio`    | `tokio::sync::RwLock`     | yes               | fair (write-preferring) |

In the above blog article, `std` was used.

### Measure performance by client
You can measure the performance of each language with the following command.

```bash
$ python client/main.py --name python --method update
{"levelname": "INFO", "asctime": "2020-03-29 09:11:08,228", "pathname": "client/main.py", "lineno": 35, "message": "receive request.", "type": "read", "elapsed": 0.003229379653930664}
{"levelname": "INFO", "asctime": "2020-03-29 09:11:08,229", "pathname": "client/main.py", "lineno": 42, "message": "receive request.", "type": "write", "elapsed": 0.00030112266540527344}
```

Since the logs are also recorded in `results` directory, you can analyze the performance of each language afterwards.
If you would like to change the experiment condition, see `python client/main.py --help`.
