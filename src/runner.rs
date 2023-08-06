use arboard::Clipboard;
use inquire::Confirm;
use tokio::process::Command;

pub async fn gh_run(args: String) {
    let exec_type = Confirm::new("Copy command to clipboard?")
        .with_default(false)
        .prompt();

    match exec_type {
        Ok(true) => {
            Clipboard::new().unwrap().set_text(args.clone()).unwrap();
        }
        Ok(false) => {
            let mut gh_process = Command::new("sh").arg("-c").arg(args).spawn().unwrap();

            let _ = gh_process.wait().await;
        }
        Err(_) => panic!("occur some errors"),
    }
}
