# Test fs wasm in docker wasmedge integration

## Build fs.wasm

```
cargo build --target wasm32-wasi --release
```

## Build docker image

```
docker buildx build --load --platform wasi/wasm -t fs:py .
```

### The structure

```
/fs.wasm
/usr
└── local
    └── lib
        ├── python3.11
        │   ├── lib-dynload
        │   └── os.py
        └── python311.zip
```

## Execute

```
docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm fs:py
```

### Current status with docker desktop v4.22.0

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: Uncategorized, error: "failed to find a pre-opened file descriptor through which \"/\" could be opened" }', src/main.rs:4:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2023-08-23 08:29:27.828] [error] execution failed: unreachable, Code: 0x89
[2023-08-23 08:29:27.828] [error]     In instruction: unreachable (0x00) , Bytecode offset: 0x00007161
```

### Expected output with docker desktop v4.20.1

```
===
List /
===
Name: /fs.wasm
Name: /etc
Name: /usr
===
List /usr
===
Name: /usr/local
===
List /usr/local
===
Name: /usr/local/lib
===
List /usr/local/lib
===
Name: /usr/local/lib/python311.zip
Name: /usr/local/lib/python3.11
```
