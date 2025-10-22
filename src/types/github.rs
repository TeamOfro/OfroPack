use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GithubReaction {
    ThumbsUp,
    ThumbsDown,
    Laugh,
    Confused,
    Heart,
    Hooray,
    Rocket,
    Eyes,
}

impl GithubReaction {
    pub const fn as_str(&self) -> &'static str {
        match self {
            GithubReaction::ThumbsUp => "+1",
            GithubReaction::ThumbsDown => "-1",
            GithubReaction::Laugh => "laugh",
            GithubReaction::Confused => "confused",
            GithubReaction::Heart => "heart",
            GithubReaction::Hooray => "hooray",
            GithubReaction::Rocket => "rocket",
            GithubReaction::Eyes => "eyes",
        }
    }
}

impl FromStr for GithubReaction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+1" => Ok(GithubReaction::ThumbsUp),
            "-1" => Ok(GithubReaction::ThumbsDown),
            "laugh" => Ok(GithubReaction::Laugh),
            "confused" => Ok(GithubReaction::Confused),
            "heart" => Ok(GithubReaction::Heart),
            "hooray" => Ok(GithubReaction::Hooray),
            "rocket" => Ok(GithubReaction::Rocket),
            "eyes" => Ok(GithubReaction::Eyes),
            _ => Err(format!("'{}' is not a valid Github reaction", s)),
        }
    }
}

impl clap::ValueEnum for GithubReaction {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            GithubReaction::ThumbsUp,
            GithubReaction::ThumbsDown,
            GithubReaction::Laugh,
            GithubReaction::Confused,
            GithubReaction::Heart,
            GithubReaction::Hooray,
            GithubReaction::Rocket,
            GithubReaction::Eyes,
        ]
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

impl serde::Serialize for GithubReaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for GithubReaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for GithubReaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
