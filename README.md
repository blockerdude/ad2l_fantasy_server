# AD2L Fantasy Server

## Setup

## Database
Postgres is the selected database.
### Installation
1) Install from this [source](https://www.postgresql.org/download/)
2) Follow instructions to setup on your local system
3) Ensure the password you have set is known or saved, it will be important further on

## Server
Rust is the language for the backend. This server is responsible for fetching information from the database and serving it up to the UI.

### Installation
1) Install Rustup from [here](https://www.rust-lang.org/tools/install)
2) If using windows you MUST install the correct build tools from [microsoft](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

    Note: Just select exactly what is listed, a restart may be required.
3) Once this is installed try building a 'hello world' project to ensure rust is working and installed correctly. Several small libraries might be required to be installed.
4) Install [Diesel and its CLI](https://diesel.rs/guides/getting-started) this is a database management tool for rust. Note [this](https://github.com/diesel-rs/diesel/issues/2519) link may help if you run into problems.
5) Follow the guide in step 4 as far as posible. You may need to mess with environment variables, or set them at a system level. Optionally and system dependent a `.env` file can be created as described below which should alert diesel and rust to the database. This has not always worked easily...

    ```
    DATABASE_URL=postgres://{{username}}:{{password}}@localhost/{{database name}}
    ```
7) Ensure that diesel can work with the database, the guide in step 4 should help assert that.


### Migrations
Once Diesel is working run the following commands

    diesel setup
     
This should instantiate the database, if it doesn't manually create it in postgres. The database can be called whatever you like 

    diesel migration run

This should will bring the database up to the current schema. 


## Development

### Migrations

To create an addition to the database run the command

    diesel migration generate {{name}}

## Running

Run the application with

    cargo run

This will launch the server which by default is listening on `localhost:8080`