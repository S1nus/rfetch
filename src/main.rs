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

struct Request {
    id: i32,
    url: String,
    status: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

fn get_url(conn: &Connection, url: hyper::Uri) {

    let my_req = Request {
        id: 0,
        url: "yahoocom".to_string(),
        status: "pending".to_string(),
        time_created: time::get_time(),
        data: None,
    };

    conn.execute(
        "INSERT INTO requests (url, status, time_created, data)
        VALUES (?1, ?2, ?3, ?4)",
        &[&my_req.url as &ToSql, &my_req.status as &ToSql, &my_req.time_created, &my_req.data],
    ).unwrap();

}

fn check_request(conn: &Connection, url: hyper::Uri) {
    let mut stmt = conn
        .prepare("SELECT id, url, status, time_created, data FROM requests WHERE url=(?)")
        .unwrap();

    let requests_iter = stmt
        .query_map(&["yahoocom".to_string()], |row| Request {
            id: row.get(0),
            url: row.get(1),
            status: row.get(2),
            time_created: row.get(3),
            data: row.get(4),
        }).unwrap();
    for request in requests_iter {
        println!("Found request {:?}", request.unwrap().url);
    }
}

fn main() {
    let conn = match Connection::open("./request_database.db") {
        Ok(conn) => conn,
        Err(e) => panic!("Couldn't connect to SQLite: {}", e),
    };

    let create_table = conn.execute(
        "CREATE TABLE requests (
            id  INTEGER PRIMARY KEY,
            url    TEXT NOT NULL,
            status    TEXT NOT NULL,
            time_created    TEXT NOT NULL,
            data    BLOB
        )",
        NO_PARAMS,
    );
    match create_table {
        Ok(_) => println!("Created table."),
        Err(e) => println!("Table seems to already exist."),
    };

    get_url(&conn, "http://google.com/".parse().unwrap());
    check_request(&conn, "http://google.com/".parse().unwrap());
}
