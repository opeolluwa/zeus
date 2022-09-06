# Zeus
A CLI chat Application client </br> Check out [zeus-svr](https://github.com/opeolluwa/zeus-svr), which is essentially the backend of this project. Also, check out [zeus-gui](https://github.com/opeolluwa/zeus-gui), A complementary GUI application that performs same functionality as this CLI app.


## Technologies 
- [Rust](https://www.rust-lang.com)
- [Clap](https://crates.io/crate/clap)
- [Axum](https://github.com/tokio-rs/axum)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/)
- [MongoDB](https://www.mongodb.com)

## Features 
 - Websockets 
 - JTW Authorization
 - Colored standard output using [console](https://crates.io/crate/console)
 - Interactive terminal with [dialoguer](https://crates.io/crate/dialoguer)

 ## Installation (Development)
 _The Application is built on [Rust Programming language](https://www.rust-lang.com). Thus, Rust is required to run the application in dev, A [MongoDB](https://www.mongodb.com) instance is also required as well as some experience with the Terminal_
 1. Clone the project `git clone https://github.com/opeolluwa/zeus`
 2. Install Dependencies `cd zeus && cargo run`
 3. Install the project on your system `cargo install --path .` This gives a binary `zeus`
 4. Exec the installed from anywhere in your terminal  `zeus`
