use std::env;
use std::io::{Error, ErrorKind};

use dotenvy::dotenv;
use env_logger::init;
use futures::executor::block_on;

use rust_orm_template::connector::db::{Db, Migration, MigrationType};

async fn run(migration_type: MigrationType) -> Result<(), Error> {
  let db = Db::new().await?;
  let migration = Migration::new(migration_type);

  if let Err(e) = db.migrate(migration).await {
    Err(Error::new(ErrorKind::Other, e.to_string()))
  } else {
    Ok(())
  }
}

fn get_migration_type(args: Vec<String>) -> Result<MigrationType, Error> {
  if args.len() < 2 {
    return Err(Error::new(ErrorKind::InvalidInput, "Migration type must be provided as an argument"));
  }

  let operation = &args[1];
  let steps: Option<u32> = if args.len() < 3 {
    None
  } else {
    match args[2].clone().parse::<u32>() {
      Ok(arg) => Some(arg),
      Err(_) => return Err(Error::new(ErrorKind::InvalidData, "steps must be a number")),
    }
  };

  match MigrationType::from_string(operation, steps) {
    Ok(r) => return Ok(r),
    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Unsupported migration type")),
  }
}

/// Run migration script
fn main() {
  dotenv().unwrap();
  init();
  let migration_type = get_migration_type(env::args().collect()).unwrap();
  if let Err(err) = block_on(run(migration_type)) {
    panic!("{}", err);
  }
}

#[cfg(test)]
mod get_migration_type_test {
  use super::get_migration_type;
  use rust_orm_template::connector::db::MigrationType;
  use std::io::ErrorKind;

  #[test]
  fn without_args() {
    match get_migration_type(vec!["migration".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert_eq!(e.to_string(), "Migration type must be provided as an argument");
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn with_invalid_type() {
    match get_migration_type(vec!["migration".to_string(), "foo".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidData);
        assert_eq!(e.to_string(), "Unsupported migration type");
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn with_refresh_type() {
    match get_migration_type(vec!["migration".to_string(), "refresh".to_string()]) {
      Ok(r) => assert_eq!(r, MigrationType::Refresh),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn with_refresh_type_and_correct_steps() {
    match get_migration_type(vec!["migration".to_string(), "refresh".to_string(), "2".to_string()]) {
      Ok(r) => assert_eq!(r, MigrationType::Refresh),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn with_refresh_type_and_incorrect_steps() {
    match get_migration_type(vec!["migration".to_string(), "refresh".to_string(), "foo".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidData);
        assert_eq!(e.to_string(), "steps must be a number");
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn with_up_type() {
    match get_migration_type(vec!["migration".to_string(), "up".to_string()]) {
      Ok(r) => assert_eq!(r, MigrationType::Up(None)),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn with_up_type_and_correct_steps() {
    match get_migration_type(vec!["migration".to_string(), "up".to_string(), "2".to_string()]) {
      Ok(r) => assert_eq!(r, MigrationType::Up(Some(2))),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn with_up_type_and_incorrect_steps() {
    match get_migration_type(vec!["migration".to_string(), "up".to_string(), "foo".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidData);
        assert_eq!(e.to_string(), "steps must be a number");
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn with_down_type() {
    match get_migration_type(vec!["migration".to_string(), "down".to_string()]) {
      Ok(r) => assert_eq!(r, MigrationType::Down(None)),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn with_down_type_and_correct_steps() {
    match get_migration_type(vec!["migration".to_string(), "down".to_string(), "2".to_string()]) {
      Ok(r) => assert_eq!(r, MigrationType::Down(Some(2))),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn with_down_type_and_incorrect_steps() {
    match get_migration_type(vec!["migration".to_string(), "down".to_string(), "foo".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidData);
        assert_eq!(e.to_string(), "steps must be a number");
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn with_negative_steps() {
    match get_migration_type(vec!["migration".to_string(), "down".to_string(), "-2".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidData);
        assert_eq!(e.to_string(), "steps must be a number");
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn with_double_steps() {
    match get_migration_type(vec!["migration".to_string(), "down".to_string(), "2.3".to_string()]) {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidData);
        assert_eq!(e.to_string(), "steps must be a number");
      },
      Ok(_) => assert!(false),
    }
  }
}
