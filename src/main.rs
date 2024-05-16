use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
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
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\\Content-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\\Content-Type: application/json\r\n\r\n";
const BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST\r\n\\Content-Type: application/json\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\\Content-Type: application/json\r\n\r\n";

// Main function

fn main(){
    // Set up database
    if let Err(e) = setup_db(){
        println!("Error setting up database: {}", e);
        return;
    }
    // Set up server
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");
    // Handle connection 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
} 

fn setup_db() -> Result<(), PostgresError> {
    let mut client = Client::connect(DATABASE_URL, NoTls)?;
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )",
        &[],
    )?;
    println!("Database set up successfully");
    Ok(())
}

// handle client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    
    match stream.read(&mut buffer){
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            let (status_line, content) = match &*request {
                req if req.starts_with("POST /users") => post_request(req),
                req if req.starts_with("GET /users/") => get_request(req),
                req if req.starts_with("GET /users") => get_all_request(req),
                req if req.starts_with("PUT /users/") => put_request(req),
                req if req.starts_with("DELETE /users/") => delete_request(req),
                _ => (NOT_FOUND.to_string(), "404 NOT FOUND".to_string()),
            };
            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// Controllers : 
// ==== POST ====
fn post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DATABASE_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2)",
                    &[&user.name, &user.email]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

// ==== GET ====
fn get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DATABASE_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}


// get_id function
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// deserialize  (deserialize mean convert from json to struct)  user from request body with the id 
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

