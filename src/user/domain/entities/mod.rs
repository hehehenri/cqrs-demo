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
            id: Uuid::new(),
            first_name,
            last_name
        }
    }

    pub fn uuid(&self) -> String {
        self.id.to_string()
    }

    pub fn first_name(&self) -> String {
        self.first_name
    }

    pub fn last_name(&self) -> String {
        self.last_name
    }
}
