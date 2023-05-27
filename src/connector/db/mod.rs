use std::env;
use std::io::{Error, ErrorKind};

use sea_orm::{DatabaseConnection, DbErr};

mod migration;
pub use migration::*;

/// Uses to handle DB connection. Initialize by running:
/// ```
/// use rust_orm_template::connector::db::Db;
/// use futures::executor::block_on;
///
/// let db = block_on(Db::new());
/// ```
#[derive(Debug)]
pub struct Db {
  _db_type: String,
  _db_user: String,
  _db_password: String,
  _db_host: String,
  _db_name: String,
  pub db_schema: String,
  pub db_url: String,
  pub connection: DatabaseConnection,
}

impl Db {
  /// Uses to initialize DB connection. ENVs must be set:
  /// * POSTGRES_USER
  /// * POSTGRES_PASSWORD
  /// * POSTGRES_HOST
  /// * POSTGRES_DB
  /// * POSTGRES_SCHEMA (default value is "default")
  #[allow(dead_code)]
  pub async fn new() -> Result<Db, Error> {
    let _db_type: String = String::from("postgres");
    let _db_user: String =
      env::var("POSTGRES_USER").or(Err(Error::new(ErrorKind::InvalidInput, "ENV POSTGRES_USER must be provided")))?;
    let _db_password: String = env::var("POSTGRES_PASSWORD")
      .or(Err(Error::new(ErrorKind::InvalidInput, "ENV POSTGRES_PASSWORD must be provided")))?;
    let _db_host: String =
      env::var("POSTGRES_HOST").or(Err(Error::new(ErrorKind::InvalidInput, "ENV POSTGRES_HOST must be provided")))?;
    let _db_name: String =
      env::var("POSTGRES_DB").or(Err(Error::new(ErrorKind::InvalidInput, "ENV POSTGRES_DB must be provided")))?;
    let db_schema: String = env::var("POSTGRES_SCHEMA").unwrap_or(String::from("default"));
    let db_url: String = format!("{}://{}:{}@{}/{}", _db_type, _db_user, _db_password, _db_host, _db_name);

    let connection: DatabaseConnection =
      Db::connect(&db_url).await.or(Err(Error::new(ErrorKind::ConnectionAborted, "error")))?;

    Ok(Db { _db_type, _db_user, _db_password, _db_host, _db_name, db_schema, db_url, connection })
  }

  #[cfg(test)]
  async fn connect(_db_url: &String) -> Result<DatabaseConnection, DbErr> {
    use sea_orm::{DatabaseBackend, MockDatabase};
    Ok(MockDatabase::new(DatabaseBackend::Postgres).into_connection())
  }

  #[cfg(not(test))]
  async fn connect(db_url: &String) -> Result<DatabaseConnection, DbErr> {
    use sea_orm::Database;
    Database::connect(db_url).await
  }

  /// User to run migration in handled DB
  #[allow(dead_code)]
  pub async fn migrate(&self, migration: Migration) -> Result<&Self, DbErr> {
    migration.run(&self.connection).await?;
    Ok(self)
  }
}

#[cfg(test)]
mod db_test {
  use super::Db;
  use futures::executor::block_on;
  use std::io::ErrorKind;
  use temp_env::with_vars;

  #[test]
  fn new_without_envs() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", None),
      ("POSTGRES_PASSWORD", None),
      ("POSTGRES_HOST", None),
      ("POSTGRES_DB", None),
      ("POSTGRES_SCHEMA", None),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert!(e.to_string().contains("POSTGRES_USER"));
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn new_without_pg_user() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", None),
      ("POSTGRES_PASSWORD", Some("pass")),
      ("POSTGRES_HOST", Some("host")),
      ("POSTGRES_DB", Some("test")),
      ("POSTGRES_SCHEMA", Some("schema")),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert!(e.to_string().contains("POSTGRES_USER"));
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn new_without_pg_password() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", Some("user")),
      ("POSTGRES_PASSWORD", None),
      ("POSTGRES_HOST", Some("host")),
      ("POSTGRES_DB", Some("test")),
      ("POSTGRES_SCHEMA", Some("schema")),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert!(e.to_string().contains("POSTGRES_PASSWORD"));
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn new_without_pg_host() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", Some("user")),
      ("POSTGRES_PASSWORD", Some("pass")),
      ("POSTGRES_HOST", None),
      ("POSTGRES_DB", Some("test")),
      ("POSTGRES_SCHEMA", Some("schema")),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert!(e.to_string().contains("POSTGRES_HOST"));
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn new_without_pg_db() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", Some("user")),
      ("POSTGRES_PASSWORD", Some("pass")),
      ("POSTGRES_HOST", Some("host")),
      ("POSTGRES_DB", None),
      ("POSTGRES_SCHEMA", Some("schema")),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Err(e) => {
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
        assert!(e.to_string().contains("POSTGRES_DB"));
      },
      Ok(_) => assert!(false),
    }
  }

  #[test]
  fn new_without_pg_schema() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", Some("user")),
      ("POSTGRES_PASSWORD", Some("pass")),
      ("POSTGRES_HOST", Some("host")),
      ("POSTGRES_DB", Some("test")),
      ("POSTGRES_SCHEMA", None),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Ok(db) => assert_eq!(db.db_url, "postgres://user:pass@host/test"),
      Err(_) => assert!(false),
    }
  }

  #[test]
  fn new_correct() {
    let kvs: [(&str, Option<&str>); 5] = [
      ("POSTGRES_USER", Some("user")),
      ("POSTGRES_PASSWORD", Some("pass")),
      ("POSTGRES_HOST", Some("host")),
      ("POSTGRES_DB", Some("test")),
      ("POSTGRES_SCHEMA", Some("schema")),
    ];

    let db = with_vars(kvs, || block_on(Db::new()));

    match db {
      Ok(db) => assert_eq!(db.db_url, "postgres://user:pass@host/test"),
      Err(_) => assert!(false),
    }
  }
}
