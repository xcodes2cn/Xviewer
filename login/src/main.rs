
use base64::encode;
use redis::Client;
use redis::Commands;
//use std::rc::Rc;
//use std::cell::RefCell;

slint::include_modules!();

pub fn main() {
    let window = LoginWindow::new().unwrap();

    let handel = window.as_weak();
    window.on_close_window(move || {
        handel.upgrade().unwrap().hide().unwrap();
        std::process::exit(0);
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
            println!("Callback: {}", my_data.borrow().uname);
            println!("Callback: {}", my_data.borrow().passwd);

            // Connect to Redis server
            let client = Client::open("redis://:redis-stack@10.8.8.2/").unwrap();
            let mut connection = client.get_connection().unwrap();
            
            // Compare passwd with key in Redis server
            let key = my_data.borrow().uname.to_string();
            let withauth = "_auth";
            let mykey = key + withauth;
            // Compare passwd with key in Redis server
            let value: String = connection.get(mykey).unwrap_or_default();

            if value == my_data.borrow().passwd {
                println!("Password matches key in Redis server!");
            } else {
                println!("Password does not match key in Redis server.");
                handel2.upgrade().unwrap().hide().unwrap();
                std::process::exit(0);
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