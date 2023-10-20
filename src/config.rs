use semver::{Version, VersionReq};
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{collections::HashMap, fmt};
use toml::Table;

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    Version::parse(value).map_err(D::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct PackageInfo {
    pub name: String,
    #[serde(deserialize_with = "deserialize_version")]
    pub version: Version,
}

#[derive(Default, Debug)]
pub struct Dependency {
    pub version: VersionReq,
}

struct DependencyVisitor;
impl<'de> Visitor<'de> for DependencyVisitor {
    type Value = Dependency;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("dependency")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Dependency {
            version: VersionReq::parse(value).map_err(E::custom)?,
            ..Default::default()
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fields = Table::new();

        while let Some((key, value)) = map.next_entry()? {
            fields.insert(key, value);
        }

        let version = VersionReq::parse(
            fields
                .get("version")
                .ok_or(A::Error::custom("expected version"))?
                .as_str()
                .ok_or(A::Error::custom("version is not string"))?,
        )
        .map_err(A::Error::custom)?;

        Ok(Dependency { version })
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DependencyVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct ProjectConfig {
    pub package: PackageInfo,
    pub dependencies: HashMap<String, Dependency>,
}
