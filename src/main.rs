use colorful::*;
use std::collections::HashMap;
use std::io;
use webbrowser;

#[tokio::main]
#[allow(unused_must_use)]
#[allow(unused_variables)]
async fn main() {
    banner();

    let mut webhook = String::new();
    let mut message = String::new();
    let mut name = String::new();
    let mut pfp = String::new();
    let mut input = String::new();
    let mut times = String::new();
    let hooksend = 0;
    let hookrem = 1;

    let opt = "Options:";
    println!("{}\nSend = 0\nDelete = 1\n\nMethod:", opt.bold());
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    if n == 0 {
        println!("\nWebhook:");
        io::stdin()
            .read_line(&mut webhook)
            .expect("I'm gonna say the nword");
        println!("\nTimes to spam:");
        io::stdin()
            .read_line(&mut times)
            .expect("I'm gonna say the nword");
        let timeint: i32 =  times.trim().parse::<i32>().expect("I'm gonna say the nword");
        println!("\nName:");
        io::stdin()
            .read_line(&mut name)
            .expect("I'm gonna say the nword");
        println!("\nProfile picture URL:");
        io::stdin()
            .read_line(&mut pfp)
            .expect("I'm gonna say the nword");
        println!("\nMessage:");
        io::stdin()
            .read_line(&mut message)
            .expect("I'm gonna say the nword");
        print!("\n");

        let client = reqwest::Client::new();

        let mut map = HashMap::new();
        map.insert("content", &message);
        map.insert("username", &name);
        map.insert("avatar_url", &pfp);

        for i in 0..timeint {
            println!("Sending: {}", i);
            client.post(&webhook).json(&map).send().await;
        }
        println!("\nFinished")
    } else if n == 1 {
        println!("\nWebhook:");
        io::stdin()
            .read_line(&mut webhook)
            .expect("I'm gonna say the nword");

        reqwest::Client::delete(&Default::default(), &webhook).send().await;
        webbrowser::open(&webhook);
        println!("\nDeleted");
    }
}

fn banner() {
    let swag = r#"██████╗  ██████╗ ██╗     ██████╗ ██╗  ██╗██╗███╗   ██╗
██╔══██╗██╔═══██╗██║     ██╔══██╗██║  ██║██║████╗  ██║
██║  ██║██║   ██║██║     ██████╔╝███████║██║██╔██╗ ██║
██║  ██║██║   ██║██║     ██╔═══╝ ██╔══██║██║██║╚██╗██║
██████╔╝╚██████╔╝███████╗██║     ██║  ██║██║██║ ╚████║
╚═════╝  ╚═════╝ ╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝"#;
    let dis = "Copyright (C) 2021 Pikkel/tty3\nThis program comes with ABSOLUTELY NO WARRANTY.\nThis is free software, and you are welcome to redistribute it under certain conditions;\nSee https://github.com/Pikkel/Dolphin/blob/main/LICENSE for full details.";
    println!("{}\n{}\n", swag.gradient(Color::Blue).bold(), dis.bold());
}
