#[macro_use]
extern crate nickel;

extern crate regex;
mod server;
mod files;

fn main() {
    let _ = files::traversal::visit(String::from("src"));
    server::server::serve();
}