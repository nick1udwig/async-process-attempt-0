use crate::exports::kinode::process::exports::Guest;
use kinode_process_lib::{await_message, call_init, println, timer::set_and_await_timer, Address, Message, Request, Response};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
    async: true,
});

fn handle_message(
    message: &Message,
) -> anyhow::Result<()> {
    println!("got Message");
    Ok(())
}

//call_init!(init);
struct Component;
impl Guest for Component {
    async fn init(_our: String) {
        println!("begin");

        async_support::spawn(async move {
            std::thread::sleep(std::time::Duration::from_secs(10));
            //set_and_await_timer(10_000);
            println!("done sleeping");
        });

        loop {
            match await_message() {
                Err(send_error) => println!("got SendError: {send_error}"),
                Ok(ref message) => match handle_message(message) {
                    Ok(_) => {}
                    Err(e) => println!("got error while handling message: {e:?}"),
                },
            }
        }
    }
}
export!(Component);
