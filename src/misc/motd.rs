use rand::Rng;

pub fn main() {
    let motd = rand::thread_rng().gen_range(0..12);
    match motd {
        0 => println!("penis balls\n"),
        1 => println!("created by a homosexual\n"),
        2 => println!("dolphin owns you\n"),
        3 => println!("snowden did nothing wrong\n"),
        4 => println!("qwaranou likes boys\n"),
        5 => println!("I need a drink\n"),
        6 => println!("Send me the Jason file\n"),
        7 => println!("Better than STYX\n"),
        8 => println!("i eat dirt and worm\n"),
        9 => println!("2+2=5\n"),
        10 => println!("pls no leak :^(\n"),
        11 => println!("use code throatcancer when buying rusherhack\n"),

        _ => {}
    }
}
