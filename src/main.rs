mod misc;
mod webhook;
pub mod config;

use colored::*;
use config::config as conf;
use misc::banner as banner;
use misc::clear as clear;
use misc::help as help;
use misc::motd as motd;
use std::io::Write;
use std::thread;
use webhook::hookcheck as hookcheck;
use webhook::hookdelete as hookdelete;
use webhook::hookfile as hookfile;
use webhook::hookspam as hookspam;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

fn main() {
    banner::main();
    motd::main();

    loop {
        let input = prompt("┌[DOLPHIN]\n└──╼$ ");
        match input.as_str() {
            "printconfig" => { conf::printconfig(); }
            "makeconfig" => { conf::makeconfig(); }
            "hookspam" => { hookspam::main(); }
            "hookdelete" => { hookdelete::main(); }
            "hookfile" => { hookfile::file(); }
            "check" => { hookcheck::browser(); }
            "clear" => { clear::main(); }
            "banner" => {
                banner::main();
                motd::main();
            }
            "b" => {
                banner::main();
                motd::main();
            }
            "help" => { help::help(); }
            "nuke" => { nuker::bot::main(); }
            "exit" => { break; }
            "x" => { break; }

            _ => println!("that's not a command idiot, type `help` for a list commands")
        }
    }
}
