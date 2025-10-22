use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemModelParent {
    Generated,
    Handheld,
}

impl ItemModelParent {
    pub const fn as_json_str(&self) -> &'static str {
        match self {
            ItemModelParent::Generated => "minecraft:item/generated",
            ItemModelParent::Handheld => "minecraft:item/handheld",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            ItemModelParent::Generated => "generated",
            ItemModelParent::Handheld => "handheld",
        }
    }
}

impl FromStr for ItemModelParent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "generated" => Ok(ItemModelParent::Generated),
            "handheld" => Ok(ItemModelParent::Handheld),
            _ => Err(format!("'{}' is not a valid ItemModelParent", s)),
        }
    }
}

impl clap::ValueEnum for ItemModelParent {
    fn value_variants<'a>() -> &'a [Self] {
        &[ItemModelParent::Generated, ItemModelParent::Handheld]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };
        <Self as FromStr>::from_str(&input)
    }
}

impl serde::Serialize for ItemModelParent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_json_str())
    }
}

impl<'de> serde::Deserialize<'de> for ItemModelParent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "minecraft:item/generated" => Ok(ItemModelParent::Generated),
            "minecraft:item/handheld" => Ok(ItemModelParent::Handheld),
            _ => Err(serde::de::Error::custom(format!(
                "'{}' is not a valid ItemModelParent",
                s
            ))),
        }
    }
}
