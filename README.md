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
$ cd rust
rust$ cargo run --release
    Finished release [optimized] target(s) in 0.05s
     Running `target/release/rust`
```


### Measure performance by client
You can measure the performance of each language with the following command.

```bash
$ python client/main.py --name python --method update
{"levelname": "INFO", "asctime": "2020-03-29 09:11:08,228", "pathname": "client/main.py", "lineno": 35, "message": "receive request.", "type": "read", "elapsed": 0.003229379653930664}
{"levelname": "INFO", "asctime": "2020-03-29 09:11:08,229", "pathname": "client/main.py", "lineno": 42, "message": "receive request.", "type": "write", "elapsed": 0.00030112266540527344}
```

Since the logs are also recorded in `results` directory, you can analyze the performance of each language afterwards.
If you would like to change the experiment condition, see `python client/main.py --help`.
