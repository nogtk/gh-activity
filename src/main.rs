mod builder;
mod runner;

#[tokio::main]
async fn main() {
    runner::gh_run(builder::gh_build()).await;
}
