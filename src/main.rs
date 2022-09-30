use actix_web::*;
mod routes;
use routes::ping::*;
use routes::info::*;
use routes::catalogo::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new( || {
        App::new()
        .route("/", web::get().to(ping))
        .route("/info", web::get().to(info))
        .route("/cat", web::get().to(catalogo))
    });

    let port = 9091;
    let api = api.bind(format!("127.0.0.1:{}", port))
    .expect("Não foi possível conectar ...");

    println!("Conectado com sucesso!\n✅ -> http://localhost:{}/", port);

    api.run()
    .await
}

// fn main() -> Result<(), Error> {
//     let mut client = Client::connect(
//         "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
//         NoTls,
//     )?;

//     // Create Table
//     client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS users (
//             id              SERIAL PRIMARY KEY,
//             username        VARCHAR UNIQUE NOT NULL,
//             password        VARCHAR NOT NULL,
//             email           VARCHAR UNIQUE NOT NULL
//             )
//     ",
//     )?;
//     println!("Create Database");

//     // Insert
//     client.execute(
//         "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
//         &[&"user2", &"mypass2", &"use2@gmail.com"],
//     )?;
//     client.execute(
//         "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
//         &[&"user3", &"anotherpass", &"mister3@test.com"],
//     )?;
//     println!("Insert Data");

//     // Update
//     // client.execute(
//     //     "UPDATE app_user SET username=$1 WHERE id=$2",
//     //     &[&"jack1", &2],
//     // )?;

//     // Delete
//     // client.execute("DELETE FROM app_user WHERE id=$1", &[&1])?;
//     // client.execute("DELETE FROM app_user WHERE id=$1", &[&3])?;

//     Ok(())
// }

// for row in client.query("SELECT id, username, password, email FROM app_user", &[])? {
//     let id: i32 = row.get(0);
//     let username: &str = row.get(1);
//     let password: &str = row.get(2);
//     let email: &str = row.get(3);
//     println!(
//         "found app user: {}) {} | {} | {}",
//         id, username, password, email
// );