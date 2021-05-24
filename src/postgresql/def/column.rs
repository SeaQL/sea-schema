#[cfg(feature="with-serde")] use serde::{Serialize, Deserialize};

use super::Type;
use super::constraints;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ColumnInfo {
    /// The name of the column
	pub name: String,
	/// The type of the column with any additional definitions such as the precision or the character
	/// set
	pub col_type: ColumnType,
	/// The default value experssion for this column, if any
	pub default: Option<ColumnExpression>,
	/// The generation expression for this column, if it is a generated colum
	pub generated: Option<ColumnExpression>,
	/// A constraint that checks whether the value of a column satisfies a Boolean expression
	pub check: Option<constraints::Check>,
	/// A constraint that ensures the value of the column is not null
	pub not_null: Option<constraints::NotNull>,
	/// A constraint that ensures the value of a column is unique among all other rows in the table
	pub unique: Option<constraints::Unique>,
	/// A constraint that states that the column is the unique identifier or part of the unique
	/// identifier of each row for this table
	pub primary_key: Option<constraints::PrimaryKey>,
	/// A constraint that ensures that the value of this column must refer to a unique key in another
	/// table
	pub references: Option<constraints::References>,
	/// Comments on the column made by the user
	pub comment: String,
}

pub type ColumnType = Type;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ColumnExpression(String);

