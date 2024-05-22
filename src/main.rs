// src/main.rs
use std::process;
use viuer::{print_from_file, Config};


fn get_terminal_size(terminal_w: &mut usize, terminal_h: &mut usize) {

    use crossterm::terminal::*;
    enable_raw_mode();
    match xterm_query::query("\x1b[14t", 10000 as u64) {
        Ok(content) => println!("terminal size pixel: {:?}", content),
        Err(err) => eprintln!("Failed to read terminal size: {}", err),
    }
    disable_raw_mode();

    // 結果を表示
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        *terminal_w = w;
        *terminal_h = h;
    } else {
        println!("Unable to get term size :(");
        process::exit(0x0100);
    }
}

fn showimg(terminal_w: &usize, terminal_h: &usize) {
    let conf = Config {
        // set offset
        x: 0,
        y: 0,
        // set dimensions
        width: Some(*terminal_w as u32),
        height: Some(*terminal_h as u32),
        ..Default::default()
    };

    print_from_file("./test/specialweek_icon.png", &conf).expect("Image printing failed.");
}

fn main() {
    let mut terminal_w : usize = 0;
    let mut terminal_h : usize = 0;
    get_terminal_size(&mut terminal_w, &mut terminal_h);
    // showimg(&terminal_w, &terminal_h);
}
