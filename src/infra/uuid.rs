use std::str::FromStr;

#[derive(Debug)]
pub struct Uuid {
    value: uuid::Uuid
}

impl TryFrom<String> for Uuid {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let result = uuid::Uuid::from_str(value.as_str());

        match result {
            Ok(uuid) => Ok(Self { value: uuid }),
            Err(error) => Err(error)
        }
    }
}

impl Uuid {
    pub fn new() -> Self {
        Self {
            value: uuid::Uuid::default()
        }
    }
}

impl ToString for Uuid {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
