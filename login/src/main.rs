
//use anyhow::Result;
//use slint::LogicalPosition;


slint::include_modules!();

/* 
fn main() -> Result<()> {
    let main = Main::new()?;

    let handel = main.as_weak();
    main.on_close_window(move ||{
        handel.upgrade().unwrap().hide().unwrap();
    });

    main.global::<Display>().xview_display(move ||{
        call_display_viewer::run();
    });
    /* 
    let handel = main.as_weak();
    main.on_minimized_window(move |enable|{
        handel.upgrade().unwrap().window().set_minimized(enable);
    });

    let handel = main.as_weak();
    main.on_maximized_window(move |enable|{
        handel.upgrade().unwrap().window().set_maximized(enable);
    });

    let handel = main.as_weak();
    main.on_move_window(move |offset_x, offset_y|{
        let main = handel.upgrade().unwrap();
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
    });
    */
    main.run()?;
    Ok(())
}
*/

pub fn main() { 
    let window = LoginWindow::new().unwrap();

    let handel = window.as_weak();
    window.on_close_window(move ||{
        handel.upgrade().unwrap().hide().unwrap();
    });

    window.global::<Display>().on_xview_display(move || {
        call_display_viewer::run();
        //std::process::exit(1);
     });
    
    window.run().unwrap();

}

mod call_display_viewer {
    // 调用外部命令
    use std::path::Path;
    use std::process::Command;

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