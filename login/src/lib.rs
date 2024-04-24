
pub mod ui {
    slint::include_modules!();
}

use base64::encode;
use base64::decode;
use redis::Client;
use redis::Commands;
use chrono::{NaiveDate, NaiveDateTime};
use slint::*;
use std::fs::File;
use std::fmt;
use std::io::Write;

// 引用controllers目录代码
mod controllers {
    pub mod header;
}

use ui::*;
use controllers::*;

pub fn main() {
    let window = MyLoginWindow::new().unwrap();

    // let _ to keep the timer alive.
    let _timer = header::setup(&window);

    let handel = window.as_weak();
    window.on_close_window(move || {
        handel.upgrade().unwrap().hide().unwrap();
        std::process::exit(0);
    });

    // hidden_window
    let weak_window = window.as_weak();
    window.on_hidden_window(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });
     
    let handel2 = window.as_weak();
    #[derive(Default)]
    struct MyData {
        uname: String,
        passwd: String,
    }
    let my_data = std::rc::Rc::new(std::cell::RefCell::new(MyData::default()));
    
    window.on_login_check({
        let my_data = my_data.clone();
        move |text, text2| {
            let encoded_passwd = encode(text2.as_bytes());

            my_data.borrow_mut().uname = text.to_string();
            my_data.borrow_mut().passwd = encoded_passwd;
            //println!("Callback: {}", my_data.borrow().uname);
            //println!("Callback: {}", my_data.borrow().passwd);

            // Connect to Redis server
            let client = Client::open("redis://:redis-stack@10.8.8.2/").unwrap();
            let mut connection = client.get_connection().unwrap();
            
            // Compare passwd with key in Redis server
            let key = my_data.borrow().uname.to_string();
            let withauth = "_auth";
            let mykey = key.clone() + withauth;
            // Compare passwd with key in Redis server
            let value: String = connection.get(mykey).unwrap_or_default();

            if !value.is_empty() && value == my_data.borrow().passwd {
                println!("Password matches key in Redis server!");
            } else {
                println!("Password does not match key in Redis server.");
                handel2.upgrade().unwrap().hide().unwrap();
                std::process::exit(0);
            }

            // 获取授权vm
            let members: Vec<String> = connection.smembers(key.clone()).unwrap();
            let num_members = members.len();
            let file = File::create("/tmp/.uri").unwrap();
            let mut writer = std::io::BufWriter::new(file);
        
            println!("Number of members in {}: {}", key, num_members);
            for member in members {
                //println!("Member: {}", member);
                let uri: String = connection.get(member.clone()).unwrap_or_default();
                println!{"{} {}", member, uri};
                write!(writer, "{} {}\n", member, uri).unwrap();

            }
        }
    });

    //let handel2 = window.as_weak();
    window.global::<Display>().on_xview_display(move || {
        call_display_viewer::run();
    });

    window.run().unwrap();
}

mod call_display_viewer {
    // 调用外部命令
    use std::path::Path;
    use std::process::Command;

    //pub fn run(){
    pub fn run(){
        //let program = "/usr/bin/open";
        let program = "/Users/mgr/src/rust/xview-lander/target/debug/xview_lander";
        let path = Path::new(program);
        if !path.is_file() {
            std::process::exit(1);
        }
        //let args = vec!["/Applications/Linphone.app"];
        let args = vec![""];
        Command::new(program)
            .args(args)
            .spawn()
            .expect(&format!("Failed to start external executable! ({program})"))
            .wait()
            .expect(&format!("Failed to wait external executable! ({program})"));
    }
}
