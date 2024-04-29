#! /bin/bash

cargo build -r --target=x86_64-pc-windows-gnu
cargo build -r --target=x86_64-unknown-linux-gnu

rm -f _sha3.exe
rm -f _sha3
cp target/x86_64-pc-windows-gnu/release/_sha3.exe .
cp target/x86_64-unknown-linux-gnu/release/_sha3 .

# sudo cp _sha3 /usr/bin