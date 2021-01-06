
use std::sync::Once;

static INIT: Once = Once::new();

// use futures::SinkExt;
pub fn setup() {

    INIT.call_once(|| {
        // setup code specific to your library's tests would go here
        println!("XXXXX Setup Tests XXXXXX");
        std::env::set_var("RUST_LOG", "actix_http=trace");
        // env_logger::init();
        env_logger::try_init().unwrap_or_else(|_err| println!("logger already initialized"));
    })
}