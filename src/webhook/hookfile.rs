use std::fs;
use std::io;

#[tokio::main]
pub async fn file() {
    let mut webhook = String::new();
    let mut path = String::new();

    println!("\nWebhook: ");
    io::stdin().read_line(&mut webhook).expect("bad");

    println!("\nFile path: ");
    io::stdin().read_line(&mut path).expect("path is fucked");

    let data = fs::read(path.clone().trim()).expect("couldn't read");

    let part = reqwest::multipart::Part::bytes(data).file_name(path);

    let form = reqwest::multipart::Form::new().part("file", part);

    reqwest::Client::new()
        .post(&webhook)
        .multipart(form)
        .send()
        .await
        .expect("didn't send");
}
