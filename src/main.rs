mod models;
mod cli;
mod export;
mod db;

fn main() {
    db::schema::define();
    cli::handler::main_loop();
}