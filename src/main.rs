use auth::config;

#[tokio::main]
async fn main() {
    let conf = config::Config::build("config/config.local.yaml")
        .expect("failed to parse config");

    auth::run(conf)
        .await
        .expect("error while running auth service");
}
