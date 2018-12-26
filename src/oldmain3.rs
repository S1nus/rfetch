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

use time::Timespec;

#[derive(Debug)]
struct Request {
    id: i32,
    url: String,
    status: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = match Connection::open("./requests_database.db") {
        Ok(conn) => conn,
        Err(e) => panic!("Could not open SQLite database :("),
    };

    let table_create = conn.execute(
        "CREATE TABLE requests (
            id  INTEGER PRIMARY KEY,
            url TEXT,
            status TEXT,
            data BLOB
        )",
        NO_PARAMS);

    match table_create {
        Ok(_) => println!("Table created."),
        Err(_) => println!("Table appears to already exist."),
    };

    addRequest(&conn);
    search_for_request(&conn);
}

fn addRequest(conn: &Connection) {
    let myRequest = Request {
        id: 0,
        url: "googlecom".to_string(),
        status: "pending".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO requests (url, status, data)
        VALUES (?1, ?2, ?3)",
        &[&myRequest.url, &myRequest.status, &"None".to_string()]
    ).unwrap();
}

fn search_for_request(conn: &Connection) {
    let mut stmt = conn
        .prepare("SELECT * FROM requests")
        .unwrap();
    let requests_iter = stmt
        .query_map(NO_PARAMS, |row| Request {
            id: row.get(0),
            url: row.get(1),
            status: row.get(2),
            data: row.get(3),
        });
    match requests_iter {
        Ok(_) => println!("It worked"),
        Err(e) => println!("error: {}", e),
    }
    for row in requests_iter {
        println!("Found request {}", row.unwrap());
    }
}
