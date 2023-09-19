use rand::Rng;

pub fn main() {
    let motd = rand::thread_rng().gen_range(0..3);
    match motd {
        0 => println!("Send me the Jason file\n"),
        1 => println!("Better than STYX\n"),
        2 => println!("i eat dirt and worm\n"),
        3 => println!("2+2=5\n"),
        _ => {}
    }
}
