// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT
use std::fs::File;
use std::io::Read;
use slint::{ModelRc, VecModel};

fn read_file_to_line(path: &str) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}

/* 备份read_line单行模式 
fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn read_line(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().to_string()
}

*/

slint::include_modules!();

pub fn main() {
    let path = "/tmp/.uri";
    
    /* 备份read_line单行模式 
    let contents = read_file(path);
    println!("{}", contents);
    let line = read_line(path);
    println!("{}", line);
    let words: Vec<&str> = line.split(" ").collect();
    println!("{:?}", words);
    let second_word = words[1];
    let fourth_word = words[3];
    println!("Second word: {}", second_word);
    println!("Fourth word: {}", fourth_word);

    let new_words: Vec<&str> = vec![words[1], words[3]];
    println!("{:?}", new_words);
    */

    let contents = read_file_to_line(path);
    println!("{:?}", contents);
    
    //
    print!("这里是测试输出 ......");

    // 构建.rs数据模型，用来存储UI界面数据
    let mut guestos_data_vec: Vec<guestos_data> = Vec::new();

    for line in contents {
        let words: Vec<&str> = line.split(" ").collect();
        println!("{:?}", words);
        let second_word = words[1];
        let fourth_word = words[3];
        println!("Second word: {}", second_word);
        println!("Fourth word: {}", fourth_word);
    
        let new_words: Vec<&str> = vec![words[1], words[3]];
        println!("{:?}", new_words);
    
        guestos_data_vec.push(guestos_data {
            gname: fourth_word.into(),
            guri: second_word.into(),
        });
    }

    // 完成.rs数据到.slint数据模型转换
    let guestos_data_model: ModelRc<guestos_data> = ModelRc::new(VecModel::from(guestos_data_vec));

    let window = MainWindow::new().unwrap();
    let week = window.as_weak();
    let res=week.upgrade().unwrap();

    // let default_myguestos = res.get_myguestos();
    // 给UI界面赋值
    res.set_myguestos(guestos_data_model);
    
    window.global::<Client>().on_xview_connect({move |guri, gname| {
        //call_remote_viewer::run()
        
        let args = vec![guri, gname];
        let args_slice: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        println!("Args: {:?}", args_slice);

        call_remote_viewer::connect_with_args(&args_slice);

     }});

    window.run().unwrap();

}

mod call_remote_viewer {
    // 调用外部命令
    use std::path::Path;
    use std::process::Command;
   
    pub fn connect_with_args(args: &Vec<&str>) {
        let program = "/bin/remote-viewer";
        let path = Path::new(program);
        if!path.is_file() {
            std::process::exit(1);
        }
        let args = vec![args[0], "-t", args[1],  "-f"];
        Command::new(program)
           .args(args)
           .spawn()
           .expect(&format!("Failed to start external executable! ({program})"))
           .wait()
           .expect(&format!("Failed to wait external executable! ({program})"));
    }

}

/* 
// 可用代码
// Mac系统打开app实例

mod call_remote_viewer {
    // 调用外部命令
    use std::path::Path;
    use std::process::Command;

    pub fn run(){
        let program = "/usr/bin/open";
        let path = Path::new(program);
        if !path.is_file() {
            std::process::exit(1);
        }
        let args = vec!["/Applications/Linphone.app"];
        Command::new(program)
            .args(args)
            .spawn()
            .expect(&format!("Failed to start external executable! ({program})"))
            .wait()
            .expect(&format!("Failed to wait external executable! ({program})"));
    }
}

*/


