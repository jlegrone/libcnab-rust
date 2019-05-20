impl<'de> Deserialize<'de> for Destination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match UncheckedDestination::deserialize(deserializer)?.validate() {
            Ok(dest) => Ok(dest),
            _ => Ok(Destination::Env("foo".to_owned())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UncheckedDestination {
    env: Option<String>,
    path: Option<PathBuf>,
}

impl UncheckedDestination {
    fn validate(self) -> Result<Destination, BundleParseError> {
        match self {
            UncheckedDestination {
                env: Some(env),
                path: None,
            } => Ok(Destination::Env(env)),
            UncheckedDestination {
                env: None,
                path: Some(path),
            } => Ok(Destination::Path(path)),
            UncheckedDestination {
                env: Some(env),
                path: Some(path),
            } => Ok(Destination::EnvAndPath(env, path)),
            UncheckedDestination {
                env: None,
                path: None,
            } => Err(BundleParseError::SerdeJSONError(serde::de::Error::custom(
                "env or path is required",
            ))),
        }
    }
}

impl serde::ser::Serialize for Destination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Destination::Env(env) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("env", &env)?;
                map.end()
            }
            Destination::Path(path) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("path", &path)?;
                map.end()
            }
            Destination::EnvAndPath(env, path) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("env", &env)?;
                map.serialize_entry("path", &path)?;
                map.end()
            }
        }
    }
}
