use std::io;
use webbrowser;

#[allow(unused_must_use)]
pub fn browser() {
    let mut webhook = String::new();

    println!("\nWebhook:");
    io::stdin()
        .read_line(&mut webhook)
        .expect("I'm gonna say the nword");

    webbrowser::open(&webhook);
    println!("should've opened in browser")
}
