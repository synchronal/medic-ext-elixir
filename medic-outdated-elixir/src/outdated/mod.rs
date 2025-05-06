use regex::Regex;

#[derive(Debug)]
pub struct Dependency {
    pub current: String,
    pub latest: String,
    pub name: String,
    pub status: String,
}

impl Dependency {
    pub(crate) fn new(name: &str, current: &str, latest: &str, status: &str) -> Self {
        Self {
            current: current.to_owned(),
            latest: latest.to_owned(),
            name: name.to_owned(),
            status: status.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct OutdatedInfo {
    pub dependencies: Vec<Dependency>,
}

impl OutdatedInfo {
    pub fn from_hex_outdated(string: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut deps = vec![];
        let dep_re =
            Regex::new(r"(?<name>[^\s]+)\s+(?<current>\d+\.\d+\.\d+[^\s]*)\s+(?<latest>[^\s]+)\s+(?<status>Update possible)")
                .unwrap();

        for line in string.lines() {
            if let Some(captures) = dep_re.captures(line) {
                let current = captures.name("current").unwrap().into();
                let latest = captures.name("latest").unwrap().into();
                let name = captures.name("name").unwrap().into();
                let status = captures.name("status").unwrap().into();
                deps.push(Dependency::new(name, current, latest, status));
            }
        }

        Ok(Self { dependencies: deps })
    }
}
