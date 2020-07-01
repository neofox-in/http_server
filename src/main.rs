mod server;
mod files;

fn main() {
    let _ = files::traversal::visit();
}