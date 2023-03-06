use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get<'a>(&'a self, key: &'_ str) -> Result<&'a str> {
        self.data.get(key).map(|s| &**s).ok_or(Error::DoesNotExist)
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<()> {
        if self.data.contains_key(&key) {
            return Err(Error::Exists);
        }

        self.data.insert(key, value);
        Ok(())
    }

    pub fn update(&mut self, key: &str, value: String) -> Result<()> {
        if !self.data.contains_key(key) {
            return Err(Error::DoesNotExist);
        }

        let curr = self.data.get_mut(key).unwrap();
        *curr = value;
        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        if !self.data.contains_key(key) {
            return Err(Error::DoesNotExist);
        }
        self.data.remove(key);
        Ok(())
    }
}

pub enum Error {
    DoesNotExist,
    Exists,
}

type Result<T> = std::result::Result<T, Error>;
