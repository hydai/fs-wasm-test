FROM scratch

COPY target/wasm32-wasi/release/fs.wasm /fs.wasm
COPY ./usr/ /usr

ENTRYPOINT [ "/fs.wasm" ]
