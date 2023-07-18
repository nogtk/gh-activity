mod author;
mod date;
mod limit;
mod output;
mod repo;

pub struct GhOption {
    pub arg: Option<String>,
    pub content: Option<String>,
}

impl GhOption {
    pub fn format(&self) -> String {
        match (&self.arg, &self.content) {
            (None, None) => String::from(""),
            (None, Some(c)) => c.to_string(),
            (Some(a), None) => a.to_string(),
            (Some(a), Some(c)) => format!("{} {}", a, c),
        }
    }
}

pub fn date() -> GhOption {
    date::build()
}

pub fn repo() -> GhOption {
    repo::build()
}

pub fn author() -> GhOption {
    author::build()
}

pub fn limit() -> GhOption {
    limit::build()
}

pub fn output() -> GhOption {
    output::build()
}
