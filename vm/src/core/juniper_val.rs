#[derive(Clone, Debug, PartialEq)]
pub enum JnpVal {
    Number(i64),
    String(String),
}

impl JnpVal {
    pub fn to_number(&self) -> Option<i64> {
        match self {
            &JnpVal::Number(n) => Some(n),
            _ => None,
        }
    }
    pub fn to_str(&self) -> Option<&str> {
        match self {
            &JnpVal::String(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<i64> for JnpVal{
    fn from(value: i64) -> Self {
        JnpVal::Number(value)
    }
}

impl<'a> From<&'a str> for JnpVal{
    fn from(value: &'a str) -> Self {
        JnpVal::String(value.to_string())
    }
}