use crate::{IdxResult,IdxError};

use serde::{Serialize,Deserialize};

#[derive(Clone,Copy,Debug,PartialEq)]
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
            Err(IdxError::storage_error_msg(format!("The name given is not a valid object name: '{}'", name)))
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
                (c != '/') &&
                (c != '\\') &&
                !char::is_control(c)
                // char::is_alphanumeric(c)
                //     || (c == '_')
                //     || (c == '-')
                //     || (c == '.')

            })
    }

    pub fn as_str(&'a self) -> &'a str {
        self.name
    }
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct ObjectNameBuf {
    name: String
}

impl ObjectNameBuf {
    pub fn new() -> Self {
        Self {
            name: String::new()
        }
    }

    pub fn from_str(name: impl AsRef<str>) -> IdxResult<Self> {
        let ptr = ObjectName::new(name.as_ref())?;
        let s = ptr.as_str().to_string();

        Ok(Self {
            name: s
        })
    }

    pub fn name<'a>(&'a self) -> ObjectName<'a> {
        ObjectName {
            name: &self.name
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(ObjectName::new("hello_world").is_ok());
        assert!(ObjectName::new("2020-05-06_22:00+0200_Linsen_mit_Saiten").is_ok());
        assert!(ObjectName::new("äöüß").is_ok());
        assert!(ObjectName::new("space is ok").is_ok());

        assert!(ObjectName::new("/").is_err());
        assert!(ObjectName::new("\\").is_err());
        assert!(ObjectName::new("\n").is_err());
    }
}