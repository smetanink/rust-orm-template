//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
  fn table_name(&self) -> &str {
    "bakery"
  }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
  pub id: i32,
  pub name: String,
  pub profit_margin: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
  Id,
  Name,
  ProfitMargin,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
  Id,
}

impl PrimaryKeyTrait for PrimaryKey {
  type ValueType = i32;
  fn auto_increment() -> bool {
    true
  }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
  Chef,
}

impl ColumnTrait for Column {
  type EntityName = Entity;
  fn def(&self) -> ColumnDef {
    match self {
      Self::Id => ColumnType::Integer.def(),
      Self::Name => ColumnType::String(None).def(),
      Self::ProfitMargin => ColumnType::Double.def(),
    }
  }
}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    match self {
      Self::Chef => Entity::has_many(super::chef::Entity).into(),
    }
  }
}

impl Related<super::chef::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Chef.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
