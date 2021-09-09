/*
  TIN - Time Tracking Application
  Copyright (C) 2021  Julio Biason

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU Affero General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Affero General Public License for more details.

  You should have received a copy of the GNU Affero General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::convert::Infallible;

use async_trait::async_trait;

/// Trait/tag for DTOs
pub trait Dto {}

/// Trait that all commands must implement.
#[async_trait]
pub trait Command<T: Dto> {
    async fn execute(&self) -> Result<&T, CommandError>;
}

/// Errors produced by invalid commands
pub enum CommandError {
    /// The request identified used in the command is already used.
    IdentifierAlreadyUsed(String),
    /// Erro with the database
    DatabaseError,
}

impl From<sqlx::Error> for CommandError {
    fn from(_error: sqlx::Error) -> Self {
        Self::DatabaseError
    }
}

impl From<Infallible> for CommandError {
    fn from(_error: Infallible) -> Self {
        Self::DatabaseError
    }
}
