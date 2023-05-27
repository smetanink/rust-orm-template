use dotenvy::dotenv;
use env_logger::init;
use log::info;

use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::prelude::*;

use rust_orm_template::connector::db::Db;

async fn run() -> Result<(), DbErr> {
  let db = Db::new().await.unwrap().connection;

  let schema_manager = SchemaManager::new(&db);

  assert!(schema_manager.has_table("bakery").await?);
  assert!(schema_manager.has_table("chef").await?);

  Ok(())
}

fn main() {
  dotenv().unwrap();
  init();
  info!("Initialized");

  if let Err(err) = block_on(run()) {
    panic!("{}", err);
  }
}
