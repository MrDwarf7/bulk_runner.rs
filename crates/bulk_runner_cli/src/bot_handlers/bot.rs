use std::collections::HashMap;

use deadpool_tiberius::tiberius::Row;

#[allow(dead_code)]
pub enum Status {
    Idle,
    Running,
    Completed,
    Pending,
    Failed,
    Unknown,
}

#[allow(dead_code)]
pub struct Bot {
    pub name: String,
    pub status: Status,
}

impl Bot {
    pub fn new(name: String, status: Status) -> Self {
        Self { name, status }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

impl From<Bot> for HashMap<String, Status> {
    fn from(bot: Bot) -> Self {
        HashMap::from([(bot.name, bot.status)])
    }
}

impl PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Bot {}

impl std::hash::Hash for Bot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl TryFrom<Row> for Bot {
    type Error = String;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let name = row
            .get::<&str, _>(0)
            .map(String::from)
            .ok_or_else(|| "Failed to get name".to_string())?;

        let status_str = row
            .get::<&str, _>(1)
            .ok_or_else(|| "Failed to get status".to_string())?;
        let status = match status_str {
            "Idle" => Status::Idle,
            "Running" => Status::Running,
            "Completed" => Status::Completed,
            _ => return Err(format!("Invalid status: {}", status_str)),
        };

        Ok(Bot::new(name, status))
    }
}
