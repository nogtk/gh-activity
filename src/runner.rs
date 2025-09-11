use arboard::Clipboard;
use inquire::Confirm;
use tokio::process::Command;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunMode {
    Clipboard,
    Exec,
}

pub async fn gh_run(args: String) {
    let on_clipboard = Confirm::new("Copy command to clipboard?")
        .with_default(false)
        .prompt();

    match on_clipboard {
        Ok(true) => gh_run_with_mode(args, RunMode::Clipboard).await,
        Ok(false) => gh_run_with_mode(args, RunMode::Exec).await,
        Err(_) => panic!("occur some errors"),
    }
}

#[allow(dead_code)]
pub async fn gh_run_with_mode(args: String, mode: RunMode) {
    match mode {
        RunMode::Clipboard => {
            Clipboard::new().unwrap().set_text(args.clone()).unwrap();
        }
        RunMode::Exec => {
            let mut gh_process = Command::new("sh").arg("-c").arg(args).spawn().unwrap();
            let _ = gh_process.wait().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, io::Write};

    #[cfg(unix)]
    #[tokio::test]
    async fn e2e_exec_runs_fake_gh_and_captures_args() {
        // 一時ディレクトリにダミー gh を作成
        let tmp = tempfile::tempdir().expect("create tempdir");
        let fake_gh = tmp.path().join("gh");
        let out_file = tmp.path().join("args.txt");

        // スクリプト: 引数を行区切りでファイルへ書き出す
        let script = format!(
            "#!/usr/bin/env bash\nset -euo pipefail\n: > '{file}'\nfor a in \"$@\"; do echo \"$a\" >> '{file}'; done\n",
            file = out_file.display()
        );
        {
            let mut f = fs::File::create(&fake_gh).unwrap();
            f.write_all(script.as_bytes()).unwrap();
        }
        // 実行権限付与
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&fake_gh).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&fake_gh, perms).unwrap();
        }

        // PATH を前置
        let orig_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", tmp.path().display(), orig_path);
        env::set_var("PATH", &new_path);

        // 実行するコマンド
        let cmd = "gh pr list --search created:2025-01-01..2025-01-31 -R owner/repo --author @me --limit 10 --json number,title".to_string();

        // 実行
        gh_run_with_mode(cmd, RunMode::Exec).await;

        // 引数記録を検証
        let recorded = fs::read_to_string(&out_file).expect("read args.txt");
        let args: Vec<&str> = recorded.lines().collect();
        assert_eq!(args[0], "pr");
        assert_eq!(args[1], "list");
        assert_eq!(args[2], "--search");
        assert_eq!(args[3], "created:2025-01-01..2025-01-31");
        assert_eq!(args[4], "-R");
        assert_eq!(args[5], "owner/repo");
        assert_eq!(args[6], "--author");
        assert_eq!(args[7], "@me");
        assert_eq!(args[8], "--limit");
        assert_eq!(args[9], "10");
        assert_eq!(args[10], "--json");
        assert_eq!(args[11], "number,title");

        // お片付け
        env::set_var("PATH", orig_path);
        tmp.close().ok();
    }
}
