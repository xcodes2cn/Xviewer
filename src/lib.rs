// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT
slint::include_modules!();

pub fn main() {
    //let login_window = LoginWindow::new().unwrap();
    //login_window.set_visible(true);
    //login_window.set_always_on_top(true);

    
    let window = MainWindow::new().unwrap();
    window.global::<Client>().on_xview_connect(move || {
        call_remote_viewer::run()
     });
    
    window.run().unwrap();

}

mod call_remote_viewer {
    // 调用外部命令
    use std::path::Path;
    use std::process::Command;

    pub fn run(){
        let program = "/bin/remote-viewer";
        let path = Path::new(program);
        if !path.is_file() {
            std::process::exit(1);
        }
        let args = vec!["spice://192.168.10.1:5907", "-t", "Kylin-desk3", "-f"];
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


