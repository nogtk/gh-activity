mod option_builder;

// テスト用に `GhOption` を外部に公開
pub use option_builder::GhOption;

pub fn gh_build() -> String {
    let mut command = "gh pr list".to_string();

    command = format!("{} {}", command, option_builder::date().format());
    command = format!("{} {}", command, option_builder::repo().format());
    command = format!("{} {}", command, option_builder::author().format());
    command = format!("{} {}", command, option_builder::limit().format());
    command = format!("{} {}", command, option_builder::output().format());

    command
}

// テスト容易性のため、任意のオプションから同等のコマンドを構築
#[allow(dead_code)]
pub fn gh_build_with(
    date: GhOption,
    repo: GhOption,
    author: GhOption,
    limit: GhOption,
    output: GhOption,
) -> String {
    let mut command = "gh pr list".to_string();
    command = format!("{} {}", command, date.format());
    command = format!("{} {}", command, repo.format());
    command = format!("{} {}", command, author.format());
    command = format!("{} {}", command, limit.format());
    command = format!("{} {}", command, output.format());
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gh_option_format_variations() {
        // (None, None) => ""
        let o = GhOption { arg: None, content: None };
        assert_eq!(o.format(), "");

        // (None, Some)
        let o = GhOption { arg: None, content: Some("value".into()) };
        assert_eq!(o.format(), "value");

        // (Some, None)
        let o = GhOption { arg: Some("--flag".into()), content: None };
        assert_eq!(o.format(), "--flag");

        // (Some, Some)
        let o = GhOption { arg: Some("--flag".into()), content: Some("value".into()) };
        assert_eq!(o.format(), "--flag value");
    }

    #[test]
    fn gh_build_with_all_options() {
        let cmd = gh_build_with(
            GhOption { arg: Some("--search".into()), content: Some("created:2025-01-01..2025-01-31".into()) },
            GhOption { arg: Some("-R".into()), content: Some("owner/repo".into()) },
            GhOption { arg: Some("--author".into()), content: Some("@me".into()) },
            GhOption { arg: Some("--limit".into()), content: Some("10".into()) },
            GhOption { arg: Some("--json".into()), content: Some("number,title".into()) },
        );

        // 末尾/間のスペースも実装に合わせてそのまま
        assert_eq!(
            cmd,
            "gh pr list --search created:2025-01-01..2025-01-31 -R owner/repo --author @me --limit 10 --json number,title"
        );
    }
}
