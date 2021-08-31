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

use crate::domain::command::Command;
use crate::domain::command::CommandError;

use super::dto::Project;

struct Create(Project);

impl Create {
    fn new(project: Project) -> Self {
        Self(project)
    }
}

impl Command<Project> for Create {
    fn execute(self) -> Result<Project, CommandError> {
        Ok(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_project() {
        let project = Project::new("project", "some project");
        let command = Create::new(project);
        assert!(command.execute().is_ok());
    }
}
