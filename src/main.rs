use regex::Regex;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::{env, fs, fs::metadata, path::PathBuf};

fn main() {
    /* let mut args: Vec<String> = env::args().collect();
    args.push(String::from("([\\w]{1,})([#\\w]{0,})"));
    args.push(String::from("D:\\Downloads\\UI_Codex\\")); */
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        //Help info
        println!("\nDelete Pics via Regex");
        println!("version 1.3\n");
        println!("This is a software used to group your files via filename (no ext) and regex, then remove duplicates with each group.\n");
        println!("Usage:");
        println!("{} <Regex> <directory>\n", &args[0]);
        println!("Example:");
        print!("{} ", &args[0]);
        print!("\"^([\\w]{{1,}})([#\\w]{{0,}})\" ");
        println!("\"D:\\Downloads\\UI_Codex\\\"\n");
        println!("The command would group the following names in \"UI_Codex_Scenery_DQ2wanglongcun\" group, and remove duplicates and small files.");
        println!("UI_Codex_Scenery_DQ2wanglongcun#55720");
        println!("UI_Codex_Scenery_DQ2wanglongcun#2349d02");
        println!("UI_Codex_Scenery_DQ2wanglongcun");

        return;
    }
    let input_regex = Regex::new(&args[1]).expect("Error regex.");
    let input_path: PathBuf = PathBuf::from(&args[2]);
    println!("{}, {:?}",input_regex,input_path);

    let (input_is_dir, checked_path) = check_path_dir(input_path);
    if !input_is_dir {
        panic!("Dir should be given.");
    }

    //开始遍历，先分组处理整合
    #[allow(unused_variables)]
    let grouped_files: Vec<Vec<PathBuf>> = path_group(&checked_path, &input_regex);

    /* //检查是否成功归类
    for m in &grouped_files {
        for n in m {
            println!("{:?}", n);
        }
        println!("---------\n");
    } */

    //按组删除
    for i in grouped_files {
        compare_delete_pics(i);
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

fn path_group(dir_path: &PathBuf, reg: &Regex) -> Vec<Vec<PathBuf>> {
    //定义正则
    //
    //输入
    //UI_Codex_Scenery_DQ2wanglongcun#55720.png
    //UI_Codex_Scenery_DQ2wanglongcun.png
    //输出group1
    //UI_Codex_Scenery_DQ2wanglongcun

    let mut i: u32 = 0;
    let mut groups: Vec<Vec<PathBuf>> = vec![vec![]];
    let mut groups_dict: HashMap<String, u32> = HashMap::new();
    for files in fs::read_dir(&dir_path).unwrap() {
        //定义了常用的各种玩意
        let each_path: PathBuf = files.unwrap().path();
        let file_name: &str = each_path.file_stem().unwrap().to_str().unwrap();

        //正则匹配（不应该出现不符和的情况的实际上）
        if reg.is_match(file_name) {
            let reg_results = reg.captures(file_name).unwrap();
            let key_name = reg_results.get(1).unwrap().as_str().to_string();

            //字典里面是否已有对应
            if !groups_dict.contains_key(&key_name) {
                //字典加入一项
                groups_dict.insert(key_name.clone(), i);
                //扩充子数组
                groups.push(vec![]);
                //字典对应的数组加入原始文件名
                groups[i as usize].push(each_path);
                i += 1;
            } else {
                let j = groups_dict.get(&key_name).unwrap().to_owned();
                groups[j as usize].push(each_path);
            }
        } else {
            println!("Error not match: {:?}", file_name);
        }
    }

    return groups;
}

fn compare_delete_pics(one_group_path: Vec<PathBuf>) {
    //主要程序判断逻辑
    if one_group_path.len() == 1 {
        return;
    }

    let mut last_size: u64 = 0;
    let mut last_path: PathBuf = PathBuf::new();
    let mut last_hash: String = String::new();

    //遍历每组内的图片并去重
    for i in one_group_path {
        let the_file_size: u64 = metadata(&i).unwrap().len();
        let the_file_hash: String = sha256_string(&i);

        if the_file_size > last_size {
            //如果新文件>旧文件，删除旧文件并将新文件的值存储
            if last_size != 0 {
                //额外判断是否是第一次循环，第一次循环不删除
                delete(&last_path);
            }
            last_path = i;
            last_size = the_file_size;
            last_hash = the_file_hash;
        } else if the_file_size < last_size {
            //如果新文件<旧文件，删除新文件
            delete(&i);
        } else {
            //如果新文件与旧文件大小相同，比较hash值
            //理论上可以在仅大小相同时才比较hash减少运算量，懒，不写了
            if last_hash == the_file_hash {
                delete(&i);
            }
        }
    }
}

fn delete(file_path: &PathBuf) {
    //实际删除函数，含错误处理
    let status = fs::remove_file(file_path);
    let _ = match status {
        Ok(()) => println!("True, delete: {:?}", &file_path),
        Err(error) => println!("Error: {}", error),
    };
}

fn sha256_string(file_path: &PathBuf) -> String {
    //hash计算函数，含错误处理
    let mut file = File::open(file_path).expect("Error open file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Error read file.");

    let mut hasher = Sha3_256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:x}", result)
}
