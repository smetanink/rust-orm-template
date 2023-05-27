use std::env;
use std::fs::remove_dir_all;
use std::io::{Error, ErrorKind};

use dotenvy::dotenv;
use env_logger::init;
use futures::executor::block_on;
use sea_orm_cli::cli::GenerateSubcommands;
use sea_orm_cli::commands::run_generate_command;
use sea_orm_cli::DateTimeCrate;

use rust_orm_template::connector::db::Db;

enum OperationType {
  Generate,
  Erase,
}

async fn generate_entities(entities_dir: String) -> Result<(), Error> {
  let db = Db::new().await?;

  if let Err(e) = run_generate_command(
    GenerateSubcommands::Entity {
      compact_format: false,
      expanded_format: true,
      include_hidden_tables: true,
      tables: Vec::new(),
      ignore_tables: vec![String::from("seaql_migrations")],
      max_connections: 10,
      output_dir: entities_dir,
      database_schema: db.db_schema,
      database_url: db.db_url,
      with_serde: String::from("none"),
      serde_skip_deserializing_primary_key: false,
      serde_skip_hidden_column: false,
      with_copy_enums: true,
      date_time_crate: DateTimeCrate::Time,
      lib: false,
      model_extra_derives: Vec::new(),
      model_extra_attributes: Vec::new(),
    },
    false,
  )
  .await
  {
    return Err(Error::new(ErrorKind::Interrupted, e.to_string()));
  }

  Ok(())
}

async fn erase_entities(entities_dir: String) -> Result<(), Error> {
  remove_dir_all(entities_dir)?;
  Ok(())
}

fn get_operation_type(args: Vec<String>) -> Result<OperationType, Error> {
  if args.len() < 2 {
    return Err(Error::new(ErrorKind::InvalidInput, "Operation type must be provided as an argument"));
  }

  let operation = &args[1];

  match operation.to_lowercase().as_str() {
    "generate" => Ok(OperationType::Generate),
    "erase" => Ok(OperationType::Erase),
    opt => Err(Error::new(ErrorKind::InvalidInput, format!("Unsupported operation {}", opt))),
  }
}

fn main() {
  dotenv().unwrap();
  init();

  let entities_dir: String = String::from("src/entities");

  match get_operation_type(env::args().collect()).unwrap() {
    OperationType::Generate => block_on(generate_entities(entities_dir)).unwrap(),
    OperationType::Erase => block_on(erase_entities(entities_dir)).unwrap(),
  }
}
