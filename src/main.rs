mod models;
mod cli;
mod export;
mod db;
mod repositories;

fn main() {
    db::setup::define();
    cli::handler::main_loop();
}