mod author;
mod date;
mod limit;
mod repo;

pub struct GhOption {
    pub arg: Option<String>,
    pub content: String,
}

impl GhOption {
    pub fn format(&self) -> String {
        match &self.arg {
            None => self.content.to_string(),
            Some(c) => format!("{} {}", c, self.content),
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
