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

use std::path::Path;
use std::path::PathBuf;

use directories::ProjectDirs;
use sqlx::sqlite::Sqlite;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;

async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    connect_with(&default_filename()).await
}

async fn connect_with(filename: &Path) -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename(filename)
                .create_if_missing(true),
        )
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

/// Return the default filename for the database.
fn default_filename() -> PathBuf {
    match ProjectDirs::from("me", "JulioBiason", "tin.sqlite3") {
        Some(project_dir) => Path::new(project_dir.config_dir()).to_path_buf(),
        None => Path::new("tin.sqlite3").to_path_buf(),
    }
}
