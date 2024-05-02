// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT
use std::fs::File;
use std::io::Read;
use slint::{ModelRc, VecModel};
use chrono::Utc;
use redis::Client;
use redis::Commands;

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
        let first_word = words[0];
        let second_word = words[1];
        let fourth_word = words[3];
        let fifth_word = words[4];
        //let sixth_word = words[5];
        println!("First word: {}", first_word);
        println!("Second word: {}", second_word);
        println!("Fourth word: {}", fourth_word);
        println!("Fifth word: {}", fifth_word);
    
        let new_words: Vec<&str> = vec![words[0], words[1], words[3], words[4]];
        println!("{:?}", new_words);
    
        guestos_data_vec.push(guestos_data {
            gid: first_word.into(),
            gname: fourth_word.into(),
            guri: second_word.into(),
            gstatus: fifth_word.into(),
        });
    }

    // 完成.rs数据到.slint数据模型转换
    let guestos_data_model: ModelRc<guestos_data> = ModelRc::new(VecModel::from(guestos_data_vec));

    // 显示用户名
    let path2 = "/tmp/.21845";
    let content2 = read_file_to_line(path2);
    println!("{:?}", content2);
    
    // 显示日期
    let now = Utc::now();
    let current_date = now.format("%Y-%m-%d").to_string();
    println!("{}", current_date);

    let window = MainWindow::new().unwrap();
    let week = window.as_weak();
    let res=week.upgrade().unwrap();

    // 给UI界面赋值
    res.set_myguestos(guestos_data_model);
    res.set_uname(content2[0].clone().into());
    //res.set_date(current_date.into());
    
    window.global::<VDIClient>().on_xview_connect({move |guri, gname| {
        //call_remote_viewer::run()
        
        let args = vec![guri, gname];
        let args_slice: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        println!("Args: {:?}", args_slice);

        call_remote_viewer::connect_with_args(&args_slice);

     }});

    // 刷新登录的虚拟机状态为：busy
    window.on_update_state(move |gid| {
        // Connect to Redis server
        let client = Client::open("redis://:redis-stack@10.8.8.2/").unwrap();
        let mut connection = client.get_connection().unwrap();

        let mykey = "state-".to_string() + gid.as_str();
        redis::cmd("SET").arg(mykey.clone()).arg("busy").execute(&mut connection);
        
        // Update the key in Redis server
        let value: String = connection.get(mykey.clone()).unwrap_or_default();
        println!("虚拟机: {}, 更新后状态: {}", mykey, value);
    });

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


