use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

// Model of Users
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// DATABASE_URL
const DATABASE_URL: &str = !env::var("DATABASE_URL").expect("DATABASE URL NOT FOUND");
// HTTP RESPONSE
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\Content-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\Content-Type: application/json\r\n\r\n";
const BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST\r\n\Content-Type: application/json\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\Content-Type: application/json\r\n\r\n";
