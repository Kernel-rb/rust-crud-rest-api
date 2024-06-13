# Rust CRUD REST API  🦀

## Overview

This project is a simple CRUD (Create, Read, Update, Delete) REST API implemented in Rust and PostgreSQL as the database. It provides endpoints to manage users in the database.

## Features

- **Create:** Add new users to the database.
- **Read:** Retrieve user information either individually or all users.
- **Update:** Modify existing user information.
- **Delete:** Remove users from the database.

## Prerequisites

Before running this application, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

## Getting Started

1. Clone this repository:
 ```bash
   git clone https://github.com/Kernel-rb/rust-crud-rest-api.git && cd rust-crud-rest-api
```
2. Build the Docker container:
```bash
    docker-compose build
```
3. Start the Docker container:
 ```bash
    docker compose up rustapp
```
4. The API server should now be running. You can access it at http://localhost:8080.

## API Endpoints
- **POST /users: Create a new user.**
- **GET /users/:id: Retrieve a user by ID.**
- **GET /users: Retrieve all users.**
- **PUT /users/:id: Update a user by ID.**
- **DELETE /users/:id: Delete a user by ID**
## Usage
You can interact with the API using tools like curl or Postman
