mod models;
mod cli;
mod export;
mod db;

fn main() {
    db::setup::define();
    cli::handler::main_loop();
}