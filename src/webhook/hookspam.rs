use std::fs::File;
use std::io;
use std::{thread, time::Duration};

#[tokio::main]
#[allow(unused_must_use)]
#[allow(unused_mut)]
pub async fn main() {
    let mut webhook = String::new();
    let mut times = String::new();
    let mut delay = String::new();

    println!("\nWebhook:");
    io::stdin()
        .read_line(&mut webhook)
        .expect("I'm gonna say the nword");

    println!("\nTimes to spam:");
    io::stdin()
        .read_line(&mut times)
        .expect("I'm gonna say the nword");
    let timeint: i32 = times
        .trim()
        .parse::<i32>()
        .expect("I'm gonna say the nword");

    println!("\nDelay:");
    io::stdin()
        .read_line(&mut delay)
        .expect("I'm gonna say the nword");
    let delayint: u64 = delay
        .trim()
        .parse::<u64>()
        .expect("I'm gonna say the nword");

    let mut file = File::open("hookconfig.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).expect("I'm gonna say the nword");

    let client = reqwest::Client::new();

    println!("");
    for i in 0..timeint {
        println!("Sending: {}", i);
        client.post(&webhook).json(&json).send().await;
        thread::sleep(Duration::from_millis(delayint));
    }
    println!("\nFinished")
}
