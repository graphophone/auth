use auth::config;

fn main() {
    let conf = config::Config::build("config/config.local.yaml")
        .expect("failed to parse config");

    auth::run(conf)
        .expect("error while running auth service");
}
