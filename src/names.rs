use crate::{IdxResult,IdxError};

#[derive(Clone,Copy)]
pub struct ObjectName<'a> {
    name: &'a str
}

impl<'a> ObjectName<'a> {
    pub fn new(name: &'a str) -> IdxResult<Self> {
        if Self::is_valid_object_name(name) {
            Ok(Self {
                name: name
            })
        } else {
            Err(IdxError::StorageError)
        }
    }

    pub fn empty() -> Self {
        Self {
            name: ""
        }
    }

    fn is_valid_object_name(name: &str) -> bool {
        name.chars()
            .all(|c| {
                char::is_alphanumeric(c)
                    || (c == '_')
                    || (c == '-')
                    || (c == '.')

            })
    }

    pub fn name(&'a self) -> &'a str {
        self.name
    }
}


