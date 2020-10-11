use std::io;
use ansi_term::Colour;

mod server;
mod directory;

fn main() -> Result<(), io::Error> {
    println!("{}  {}",
             Colour::Red.bold().paint("Started At :: "),
             Colour::Yellow.underline().paint("http:://0.0.0.0:8989"));
    server::http::serve()
}