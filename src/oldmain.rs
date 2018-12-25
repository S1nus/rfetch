extern crate futures;
extern crate hyper;
extern crate tokio;

use std::io::{self, Write};
use std::iter;
use futures::{stream, Future, Stream};
use hyper::Client;
use tokio::prelude::*;

fn get_url() {
    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse().unwrap();
    let uris = iter::repeat(uri).take(50);

    let work = stream::iter_ok(uris)
        .map(move |uri| client.get(uri))
        .buffer_unordered(1)
        .and_then(|res| {
            println!("Response: {}", res.status());
            res.into_body().concat2().from_err()
        })
        .map_err(|e| eprintln!("get error: {}", e))
        .for_each(|body| {
            io::stdout().write_all(&body).map_err(|e| {
                println!("e: {}", e);
            })
        });

    tokio::run(work);
}

fn main() {
    get_url();
}
