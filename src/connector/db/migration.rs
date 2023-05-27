use sea_orm::DatabaseConnection;
use sea_orm_migration::{DbErr, MigratorTrait};
use std::io::{Error, ErrorKind};

use crate::migrator::Migrator;

/// Uses to setup migration settings
/// * MigrationType::Refresh - Rollback all applied migrations, then reapply all migrations
/// * MigrationType::Up(steps) - Apply pending migrations. If steps "None" - all pending migration will be applied
/// * MigrationType::Down(steps) - Rollback applied migrations. If steps "None" - all applied migration will be rolled back
#[derive(Debug, PartialEq, Eq)]
pub enum MigrationType {
  Refresh,
  Up(Option<u32>),
  Down(Option<u32>),
}

/// Uses to handle migration settings. Initialize by running:
/// ```
/// use rust_orm_template::connector::db::{Migration, MigrationType};
///
/// let migration_type = MigrationType::from_string("refresh", None).unwrap();
/// let migration = Migration::new(migration_type);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Migration {
  migration_type: MigrationType,
}

impl MigrationType {
  /// Build MigrationType from string.
  /// Allowed operations: "refresh", "up" and "down" in any case, e.g. "ReFrEsH" - is possible
  #[allow(dead_code)]
  pub fn from_string(operation: &str, steps: Option<u32>) -> Result<MigrationType, Error> {
    match operation.to_lowercase().as_str() {
      "refresh" => Ok(MigrationType::Refresh),
      "up" => Ok(MigrationType::Up(steps)),
      "down" => Ok(MigrationType::Down(steps)),
      opt => Err(Error::new(ErrorKind::InvalidInput, format!("Unsupported operation {}", opt))),
    }
  }
}

impl Migration {
  /// Uses to initialize new migration.
  #[allow(dead_code)]
  pub fn new(migration_type: MigrationType) -> Migration {
    Migration { migration_type }
  }

  /// Run migration on DB
  #[allow(dead_code)]
  pub async fn run(&self, connection: &DatabaseConnection) -> Result<&Self, DbErr> {
    match &self.migration_type {
      MigrationType::Refresh => Migrator::refresh(connection).await?,
      MigrationType::Up(steps) => Migrator::up(connection, *steps).await?,
      MigrationType::Down(steps) => Migrator::down(connection, *steps).await?,
    }

    Ok(self)
  }
}

#[cfg(test)]
mod migration_test {
  use super::MigrationType;
  use std::io::ErrorKind;

  #[test]
  fn from_empty_string() {
    let result = MigrationType::from_string("", None);
    match result {
      Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput),
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn from_incorrect_value() {
    let value = "some_value";
    let result = MigrationType::from_string(value, None);
    match result {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert!(e.to_string().contains(value));
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn from_value_refresh() {
    let result = MigrationType::from_string("refresh", None);
    match result {
      Ok(value) => assert_eq!(value, MigrationType::Refresh),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn from_value_uppercase_refresh() {
    let result = MigrationType::from_string("REFRESH", None);
    match result {
      Ok(value) => assert_eq!(value, MigrationType::Refresh),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn from_value_up_none() {
    let result = MigrationType::from_string("up", None);
    match result {
      Ok(value) => assert_eq!(value, MigrationType::Up(None)),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn from_value_up_steps() {
    let result = MigrationType::from_string("up", Some(5));
    match result {
      Ok(value) => assert_eq!(value, MigrationType::Up(Some(5))),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn from_value_down_none() {
    let result = MigrationType::from_string("down", None);
    match result {
      Ok(value) => assert_eq!(value, MigrationType::Down(None)),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn from_value_down_steps() {
    let result = MigrationType::from_string("down", Some(5));
    match result {
      Ok(value) => assert_eq!(value, MigrationType::Down(Some(5))),
      Err(_) => assert!(false),
    }
  }
}
