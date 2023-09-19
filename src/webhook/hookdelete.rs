use std::io;
use webbrowser;

#[tokio::main]
#[allow(unused_must_use)]
#[allow(unused_mut)]
pub async fn main() {
    let mut webhook = String::new();

    println!("\nWebhook:");
    io::stdin()
        .read_line(&mut webhook)
        .expect("uhhhhhh");

    reqwest::Client::delete(&Default::default(), &webhook)
        .send()
        .await;
    webbrowser::open(&webhook);
    println!("\nDeleted");
}
