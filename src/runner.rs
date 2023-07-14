use tokio::process::Command;

pub async fn gh_run(args: String) {
    let mut gh_process = Command::new("sh").arg("-c").arg(args).spawn().unwrap();

    let _ = gh_process.wait().await;
}
