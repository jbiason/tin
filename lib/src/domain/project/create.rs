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

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use async_trait::async_trait;

use super::dto::Project;
use super::repository::Repository;
use crate::database;
use crate::domain::command::Command;
use crate::domain::command::CommandError;

struct Create(Project);

impl Create {
    fn new(project: Project) -> Self {
        Self(project)
    }
}

#[async_trait]
impl Command<Project> for Create {
    async fn execute(&self) -> Result<&Project, CommandError> {
        log::debug!(
            "Creating project \"{}\": \"{}\"",
            self.0.code(),
            self.0.name()
        );
        let pool = database::connect_with(PathBuf::from_str("./test.sqlite")?.as_path()).await?;
        let repo = Repository::new(&pool);
        repo.save(&self.0).await?;
        Ok(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delete_temp_database() {
        fs::read_dir(".")
            .unwrap()
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_ascii_lowercase()
                    .into_string()
                    .unwrap()
            })
            .filter(|filename| filename.starts_with("test.sqlite"))
            .for_each(|filename| fs::remove_file(filename).unwrap());
    }

    #[tokio::test]
    async fn should_create_project() {
        env_logger::init();
        delete_temp_database();
        let project = Project::new("project", "some project");
        let command = Create::new(project);
        assert!(command.execute().await.is_ok());
        delete_temp_database();
    }
}
