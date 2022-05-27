set dotenv-load
export RUST_BACKTRACE := "1"
alias d := dev

build:
    just build-web
    just build-proxy

build-web:
    cargo build --bin web --release

build-proxy:
    cargo build --bin proxy --release    


test:
    # will print a stack trace if it crashes
    cargo test

default:
    echo 'Hello, world!'

dev:
    cargo watch --exec 'run --bin web'

install:
    cargo install cargo-edit
    cargo install cargo-watch

serve:
    @echo "Starting server with host $HOST on port $PORT…"

px:
    cargo run --bin proxy -- 0.0.0.0 7778 0.0.0.0 7007

burst:
    wrk -t4 -c400 -d30s http://127.0.0.1:7007/

burst2:
    wrk -t4 -c400 -d30s http://127.0.0.1:7778/    

ping:
    watch -n 5 -c 'curl localhost:7007/counter'

