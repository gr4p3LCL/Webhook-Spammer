use std::io;
use webbrowser;

#[allow(unused_must_use)]
pub fn browser() {
    let mut webhook = String::new();

    println!("\nWebhook:");
    io::stdin()
        .read_line(&mut webhook)
        .expect("uhhhhhh");

    webbrowser::open(&webhook);
    println!("should've opened in browser")
}
