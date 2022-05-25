pub fn initiate_logging() {
    // std::env::set_var("RUST_LOG", "debug, actix_web=debug");

    std::env::set_var("RUST_LOG", "info");

    // let env = std::env::var("ADDRESS").expect("'.env' not found.");
    // dbg!(env);

    env_logger::init();
}
