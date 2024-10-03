#[derive(Debug)]

pub enum CommitType {
    Feat,
    Fix,
    Refactor,
    Perf,
    Style,
    Test,
    Docs,
    Build,
    Ops,
    Chore,
}

impl CommitType {
    pub fn new(c: &str) -> Self {
        match c {
            "feat" => CommitType::Feat,
            "fix" => CommitType::Fix,
            "refactor" => CommitType::Refactor,
            "perf" => CommitType::Perf,
            "style" => CommitType::Style,
            "test" => CommitType::Test,
            "docs" => CommitType::Docs,
            "build" => CommitType::Build,
            "ops" => CommitType::Ops,
            "chore" => CommitType::Chore,
            _ => panic!("Invalid commit type: '{}'", c),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Refactor => "refactor",
            CommitType::Perf => "perf",
            CommitType::Style => "style",
            CommitType::Test => "test",
            CommitType::Docs => "docs",
            CommitType::Build => "build",
            CommitType::Ops => "ops",
            CommitType::Chore => "chore",
        }
    }

    pub fn as_str_vec() -> Vec<&'static str> {
        vec![
            CommitType::Feat.as_str(),
            CommitType::Fix.as_str(),
            CommitType::Refactor.as_str(),
            CommitType::Perf.as_str(),
            CommitType::Style.as_str(),
            CommitType::Test.as_str(),
            CommitType::Docs.as_str(),
            CommitType::Build.as_str(),
            CommitType::Ops.as_str(),
            CommitType::Chore.as_str(),
        ]
    }
}
