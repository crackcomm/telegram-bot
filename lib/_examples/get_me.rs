extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;

use std::env;

use futures::{future::lazy, Future};

use telegram_bot_fork::*;

fn main() {
    tokio::runtime::current_thread::Runtime::new()
        .unwrap()
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::new_default(token).unwrap();

            tokio::executor::current_thread::spawn(api.send(GetMe).then(|r| {
                println!("{:?}", r);

                Ok(())
            }));

            Ok::<_, Error>(())
        }))
        .unwrap();
}
