use lazy_static::lazy_static;
use std::{collections::HashMap, str::FromStr, convert::Infallible};

macro_rules! define_primitive {
    ($($variant:ident => $value:literal),+) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Primitive {
            $($variant),+
        }

        lazy_static! {
            static ref PRIMITIVE_MAP: HashMap<&'static str, Primitive> = {
                let mut map = HashMap::new();
                $(map.insert($value, Primitive::$variant);)+
                map
            };
        }
    }
}

define_primitive!(
    Void => "void",
    Int => "int"
);

impl Primitive {
    fn from_str(value: &str) -> Option<Self> {
        PRIMITIVE_MAP.get(value).cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(Primitive),
    UserDefined(String),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        if let Some(primitive) = Primitive::from_str(value) {
            Type::Primitive(primitive)
        } else {
            Type::UserDefined(value.to_string())
        }
    }
}

impl FromStr for Type {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Type::from(s))
    }
}