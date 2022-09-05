use clap::Parser;

//import modules
mod modules;
use modules::shell;
fn main() {
    shell::Zeus::parse();
}
