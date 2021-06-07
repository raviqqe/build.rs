#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExternalPackage {
    name: String,
    location: String,
    expected_version: String,
}

impl ExternalPackage {
    pub fn new(
        name: impl Into<String>,
        location: impl Into<String>,
        expected_version: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            location: location.into(),
            expected_version: expected_version.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn expected_version(&self) -> &str {
        &self.expected_version
    }
}
