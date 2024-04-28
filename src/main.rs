use regex::Regex;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "\nRegex QQMusic Renamer    version 1.8\n\nUsage:\n{} <directory>\n{} <files>\n",
            &args[0], &args[0]
        );
        return;
    }

    let input_path: PathBuf = PathBuf::from(&args[1]);
    let (input_is_dir, checked_path) = check_path_dir(input_path);
    if input_is_dir {
        println!("Enum {:?} ...", checked_path);
        for files in fs::read_dir(&checked_path.as_path()).unwrap() {
            rename_files(&files.unwrap().path());
        }
    } else {
        args.remove(0);
        for i in args {
            rename_files(&PathBuf::from(&i));
        }
    }
}

fn check_path_dir(mut path_to_check: PathBuf) -> (bool, PathBuf) {
    path_to_check = if path_to_check.ends_with("\"") {
        PathBuf::from(path_to_check.to_string_lossy().strip_suffix("\"").unwrap())
    } else {
        path_to_check
    };

    if !path_to_check.exists() {
        panic!("{:?} does not exist.", path_to_check.to_str());
    }

    (path_to_check.is_dir(), path_to_check)
}

fn get_new_name(regex: Regex, original_name: &str) -> &str {
    regex
        .captures(original_name)
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
}

fn rename_files(each_path: &PathBuf) {
    let reg = Regex::new(r"(.*) - (.*)( \[.*\])").unwrap();
    //File name example: "HOYO-MiX - 枫丹 Fontaine [qmmc2].flac" -> "枫丹 Fontaine.flac"
    if !each_path.is_dir() {
        let ext_list: [&str; 8] = ["ogg", "mp3", "flv", "flac", "wav", "mgg2", "mgg", "mflac"];
        let file_name = each_path.file_stem().unwrap().to_str().unwrap();
        let file_ext = each_path.extension().unwrap().to_str().unwrap();
        let dir_path = each_path.parent().unwrap().to_str().unwrap();

        if ext_list.contains(&file_ext) && reg.is_match(file_name) {
            let new_file_name = get_new_name(reg, file_name);
            let new_file_path =
                PathBuf::from(dir_path).join(new_file_name.to_string() + "." + file_ext);
            fs::rename(each_path, new_file_path).unwrap();
            println!("Renamed {} to {}", file_name, new_file_name);
        }

        let need_delete_list: [&str; 2] = ["jpg", "lrc"];
        if need_delete_list.contains(&file_ext) {
            fs::remove_file(each_path).unwrap();
        }
    }
}
