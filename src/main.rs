use sha3::{Digest, Sha3_256};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Wrong args.");
    }
    let input_path = PathBuf::from(&args[1]);
    if (!input_path.exists()) || input_path.is_dir() {
        panic!("Wrong file types.")
    }

    calc_sha3_256(&input_path);
}

fn calc_sha3_256(file_path: &PathBuf) {
    let name = file_path.file_name().unwrap().to_str().unwrap();
    let mut file = File::open(file_path).expect("无法打开文件");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("无法读取文件内容");

    let mut hasher = Sha3_256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    println!("{} SHA3_256={:x}", name, result);
}
