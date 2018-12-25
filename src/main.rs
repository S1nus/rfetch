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

fn get_url(connection : &Connection, url : hyper::Uri) {
    
    let mut query = connection.prepare("SELECT * FROM requests WHERE url=?").unwrap();
    let mut results = query.query(&[url.to_string()]);

    match results {
        Ok(res) => println!("Success!"),
        Err(e) => println!("Error: {:?}", e),
    };
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

    get_url(&conn, "https://www.reddit.com/r/trees/new.json".parse().unwrap());
}
