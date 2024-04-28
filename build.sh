cargo build -r --target=x86_64-pc-windows-gnu
cargo build -r --target=x86_64-unknown-linux-gnu

\cp target/x86_64-pc-windows-gnu/release/_sha3.exe .
\cp target/x86_64-unknown-linux-gnu/release/_sha3 .
