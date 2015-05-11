extern crate postgres;
mod the_printers;

use postgres::{Connection, SslMode};

fn main() {
    the_printers::title();

    let dsn = "postgresql://dbegin@localhost/first_rust_db";

    let conn = match Connection::connect(dsn, &SslMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("An Error: {:?}", e);
            return;
        }
    };


    println!("{:?}", conn);
}
