// #[cfg(feature="syc")]
// mod client;
// #[cfg(feature="ssr")]
// mod common;
// #[cfg(feature="ssr")]
// mod server;
// #[cfg(feature="ssr")]
// mod ssr{
//     use std::sync::Arc;

//     use actix_web::{get, web::Data, App, HttpServer, Responder};
//     use sqlx::{migrate::MigrateDatabase, query_as, sqlite::{SqliteConnectOptions, SqliteJournalMode}, Pool, Sqlite, SqlitePool};

//     use crate::{common::{AppError, Res},server::map::BigIntDB};

//     pub async fn db()->Res<Pool<Sqlite>> {
//         use std::str::FromStr;
//         use dotenv::dotenv;
//         use std::env;
//         //std::env::set_current_dir("/Users/lucas.igarzabal/VsCode/Rust/tauri-server/")?; //C:\Users\lucas.igarzabal\VsCode\Rust\tauri-server
//         //let res=std::env::current_dir()?;
//         //println!("Aca el path {}",res.display());
//         dotenv().expect(".env Not set");
//         let url = env::var("DATABASE_URL")?;
//         // let url = "sqlite://sqlite.db";
//         println!(" {} ",url);
//         if !Sqlite::database_exists(url.as_str()).await.unwrap_or(false) {
//             return Err(AppError::NotFound { objeto: "Database".to_string(), instancia: String::new() })
//         }
//         let conn =  SqliteConnectOptions::from_str(url.as_str())?.journal_mode(SqliteJournalMode::Wal).create_if_missing(true);
//         let db =  SqlitePool::connect(url.as_str()).await?;
//         db.set_connect_options(conn);
//         Ok(db)
//     }

//     #[get("/admin/get_productos")]
//     async fn get_productos(data: Data<Arc<Pool<Sqlite>>>)->impl Responder{
//         let db = data.into_inner();
//         println!("get_productos");
//         let res=query_as!(BigIntDB,"select id as int from productos").fetch_all(db.as_ref().as_ref()).await.expect("Error DB");
//         format!("{:#?}",res.iter().map(|x|x.int).collect::<Vec<i64>>()[5])
//     }

//     pub async fn main_fn()->std::io::Result<()>{
//         let db = db().await.expect("No DB");
//         HttpServer::new(move ||App::new().app_data(Data::new(db.clone())).service(get_productos)).bind(("127.0.0.1",8080))?.run().await
//     }
// }
// #[cfg(feature="syc")]
// mod syc{
    
// }


// use std::{future::Future, time::Duration};
mod client;
use client::app::App;
// use tokio::{spawn, task::JoinHandle};

fn main() {
    // #[cfg(feature="ssr")]
    // {
        
    //     tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async{ssr::main_fn().await}).unwrap();
    // }
    
    console_error_panic_hook::set_once();
    sycamore::render(App);
}

// #[cfg(all(feature="ssr",not(feature="syc")))]
// use actix_web::{get, web::{self, Data}, App, Error, HttpResponse, HttpServer, Responder};
// #[cfg(all(feature="ssr",not(feature="syc")))]
// use sqlx::{migrate::MigrateDatabase,query_as, sqlite::{SqliteConnectOptions, SqliteJournalMode}, Pool, Sqlite, SqlitePool};
// #[cfg(all(feature="ssr",not(feature="syc")))]
// use common::{AppError,Res};
// #[cfg(all(feature="ssr",not(feature="syc")))]
// pub async fn db()->Res<Pool<Sqlite>> {
//     use std::str::FromStr;
//     use dotenv::dotenv;
//     use std::env;
//     //std::env::set_current_dir("/Users/lucas.igarzabal/VsCode/Rust/tauri-server/")?; //C:\Users\lucas.igarzabal\VsCode\Rust\tauri-server
//     //let res=std::env::current_dir()?;
//     //println!("Aca el path {}",res.display());
//     dotenv().expect(".env Not set");
//     let url = env::var("DATABASE_URL")?;
//     // let url = "sqlite://sqlite.db";
//     println!(" {} ",url);
//     if !Sqlite::database_exists(url.as_str()).await.unwrap_or(false) {
//         return Err(AppError::NotFound { objeto: "Database".to_string(), instancia: String::new() })
//     }
//     let conn =  SqliteConnectOptions::from_str(url.as_str())?.journal_mode(SqliteJournalMode::Wal).create_if_missing(true);
//     let db =  SqlitePool::connect(url.as_str()).await?;
//     db.set_connect_options(conn);
//     Ok(db)
// }

// #[get("/admin/get_productos")]
// #[cfg(all(feature="ssr",not(feature="syc")))]
// async fn get_productos(data: Data<Arc<Pool<Sqlite>>>)->impl Responder{
//     let db = data.into_inner();
//     let res=query_as!(BigIntDB,"select id as int from productos").fetch_all(db.as_ref().as_ref()).await.expect("Error DB");
//     format!("{:#?}",res.iter().map(|x|x.int).collect::<Vec<i64>>()[5])
// }

// #[cfg(all(feature="ssr",not(feature="syc")))]
// mod server;
// #[cfg(all(feature="ssr",not(feature="syc")))]
// #[actix_web::main]
// async fn main()->std::io::Result<()>{
    
//     let db = db().await.expect("No DB");
//     HttpServer::new(move ||App::new().app_data(Data::new(db.clone())).service(get_productos)).bind(("127.0.0.1",8080))?.run().await
// }

