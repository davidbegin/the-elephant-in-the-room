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

    conn.execute("create table if not exists blog (
        id serial primary key,
        title varchar(255),
        body text)", &[]).ok().expect("Table creation failed");

    println!("{:?}", conn);

    conn.execute("drop table blog", &[])
        .ok().expect("Table creation failed");
}
