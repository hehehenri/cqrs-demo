use crate::infra::uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            id: Uuid::default(),
            first_name,
            last_name
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.id
    }

    pub fn first_name(&self) -> &str {
        self.first_name.as_str()
    }

    pub fn last_name(&self) -> &str {
        self.last_name.as_str()
    }
}
