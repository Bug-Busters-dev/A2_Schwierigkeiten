#[derive(Debug)]
pub enum UnionType {
    Number(u32),
    String(String),
    Vec(Vec<u32>),
}
impl UnionType {
    pub fn get_value<T>(self) -> Option<T>
    where
        T: From<UnionType>,
    {
        match T::from(self) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

pub trait From<T> {
    fn from(value: T) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl From<UnionType> for u32 {
    fn from(value: UnionType) -> Result<Self, &'static str> {
        match value {
            UnionType::Number(n) => Ok(n),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<UnionType> for String {
    fn from(value: UnionType) -> Result<Self, &'static str> {
        match value {
            UnionType::String(s) => Ok(s),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<UnionType> for Vec<u32> {
    fn from(value: UnionType) -> Result<Self, &'static str> {
        match value {
            UnionType::Vec(v) => Ok(v),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<UnionType> for usize {
    fn from(value: UnionType) -> Result<Self, &'static str> {
        match value {
            UnionType::Number(n) => Ok(n as usize),
            _ => Err("Invalid conversion"),
        }
    }
}
