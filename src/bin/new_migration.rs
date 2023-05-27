use std::env;

use dotenvy::dotenv;
use env_logger::init;

use sea_orm_cli::run_migrate_generate;

fn get_migration_name(args: Vec<String>) -> String {
  if args.len() < 2 {
    return String::from("migration");
  }

  args[1].clone()
}

fn main() {
  dotenv().unwrap();
  init();
  let name = get_migration_name(env::args().collect());
  run_migrate_generate("src/migrator", name.as_str(), true).unwrap();
}

#[cfg(test)]
mod  get_migration_type_test{
  use super::get_migration_name;

  #[test]
  fn without_name() {
    let result = get_migration_name(vec![String::from("new_migration")]);
    assert_eq!(result, String::from("migration"));
  }

  #[test]
  fn with_name() {
    let result = get_migration_name(vec![String::from("new_migration"), String::from("migration_name")]);
    assert_eq!(result, String::from("migration_name"));
  }
}