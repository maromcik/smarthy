use std::sync::atomic::AtomicBool;
use smarthy_engine::switch::SmartSwitch;


#[tokio::main]
async fn main() {

let mut x = SmartSwitch {
    current_state: AtomicBool::new(false),
    set_state: AtomicBool::new(false),
};
    
    x.poll_state().await;
    println!("Hello, world!");
}
