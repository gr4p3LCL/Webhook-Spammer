use std::io::Read;
use std::io::Write;

pub fn makeconfig() {
    let mut file = std::fs::File::create("hookconfig.json").expect("failed to create");
    file.write_all("{".as_bytes())
        .expect("failed to write config");
    file.write_all(
        "\n  \"avatar_url\": \"https://www.kernel.org/theme/images/logos/tux.png\",".as_bytes(),
    )
    .expect("failed to write config");
    file.write_all("\n  \"content\": \"Linux is better than windows\",".as_bytes())
        .expect("failed to write config");
    file.write_all("\n  \"username\": \"Tux\"".as_bytes())
        .expect("failed to write config");
    file.write_all("\n}".as_bytes())
        .expect("failed to write config");
    println!("config made");
}

pub fn printconfig() {
    let mut file = std::fs::File::open("hookconfig.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}\n", contents);
}
