use crate::Error;

#[derive(Debug)]
pub struct Repo {
    pub name: String,
    pub repo: String,
    pub note: String,
    pub readme: Option<String>,
    pub private: bool,
}

#[derive(Debug)]
pub struct Commit {
    pub author: Author,
    pub timestamp: i64, // Change to date
    pub summary: String,
    pub oid: Oid,
    // Idk really now how this works
    // but I probably dont want to
    // store all the diffs in here
    // because we're caching this
    // and if there are a lot of
    // commit's we run into
    // memory concerns
}

impl std::convert::From<git2::Commit<'_>> for Commit {
    fn from(commit: git2::Commit<'_>) -> Self {
        Self {
            author: commit.author().into(),
            timestamp: commit.time().seconds(),
            summary: commit.summary().unwrap_or("[NON UTF-8 SUMMARY]").to_string(),
            oid: commit.id().into(),
        }
    }
}

#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl std::convert::From<git2::Signature<'_>> for Author {
    fn from(signature: git2::Signature) -> Self {
        Self {
            name: signature.name().unwrap_or("[NON UTF-8 NAME]").to_string(),
            email: signature.email().unwrap_or("[NON UTF-8 EMAIL]").to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub size: Option<usize>,
    pub oid: Oid,
    pub children: Option<Vec<TreeNode>>,
}

#[derive(Clone, PartialEq)]
pub struct Oid(Vec<u8>);

impl Oid {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::str::FromStr for Oid{
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if s.len() != 40 { return Err(Error::ParseOidError("Incorrect length".to_string())) };
        let bytes: Vec<u8> = (0..s.len())
            .step_by(2)
            .filter_map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
            .collect();
        Ok(Self(bytes))
    }
}

impl std::convert::From<git2::Oid> for Oid {
    fn from(oid: git2::Oid) -> Self {
        Self(oid.as_bytes().to_vec())
    }
}

impl std::fmt::Debug for Oid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for Oid {
    fn to_string(&self) -> String {
        self.0.iter()
            .map(|n| format!("{:02x}", n))
            .collect()
    }
}
