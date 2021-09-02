/*
  TIN - Time tracking application
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

use sqlx::sqlite::Sqlite;
use sqlx::Pool;

use super::dto::Project;

struct Repository<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> Repository<'a> {
    fn new(pool: &'a Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn save(&self, project: Project) -> Result<Project, sqlx::Error> {
        sqlx::query(r#"INSERT INTO project (code, name) VALUES (?, ?)"#)
            .bind(&project.code())
            .bind(&project.name())
            .execute(self.pool)
            .await?;
        Ok(project)
    }
}
