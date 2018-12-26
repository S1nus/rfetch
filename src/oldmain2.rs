extern crate futures;
extern crate hyper;
extern crate tokio;

use std::io::{self, Write};
use std::iter;
use std::path::Path;

use futures::{stream, Future, Stream};
use hyper::Client;
use tokio::prelude::*;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};

struct Query {
    usize: id,
    String: time_created,
    String: url,
    String: status,
    data,
}

fn get_url(connection : &Connection, url : hyper::Uri) {
}

fn main() {
    println!("Attempting to open SQLite Database...");

    let conn = match Connection::open(Path::new("./query_database.db")) {
        Ok(conn) => conn,
        Err(error) => {
            panic!("Error opening SQLite database: {:?}", error);
        },
    };

    let table_create = conn.execute(
        "CREATE TABLE requests (
            id INTEGER PRIMARY KEY,
            time_created TEXT NOT NULL,
            url TEXT,
            status TEXT,
            data TEXT
        )",
        NO_PARAMS,
    );

    match table_create {
        Ok(size) => println!("Table created"),
        Err(_) => println!("Table appears to already exist"),
    };

    get_url(&conn, "poop".parse().unwrap());
}
