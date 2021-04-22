
use log::*;
use std::io::Write;

mod game;
use game::{AppContext};

const BOT_TOKEN: &str = "1788123506:AAE7T_V3Nr4dGjdUL5pD4Yq-w_5KN1cDB8g";

fn init_logger() {

    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env().format(move |buf, rec| {
        let t = start.elapsed().as_secs_f32();
        writeln!(buf, "{:.03} [{}] - {}", t, rec.level(),rec.args())}
    ).init();
}

fn main() {
    
    init_logger();

    let app_context = AppContext::new(BOT_TOKEN);

    let  rt = tokio::runtime::Runtime::new().expect("Couldn't run a runtime thread");
    let future = app_context.main_loop.run();
    let result = rt.block_on(future);

    if let Err(err_text) = result {
        error!("Thread finished with error: {}.", err_text);
    }
}