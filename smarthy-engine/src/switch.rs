use std::sync::atomic::AtomicBool;
use tokio::time::sleep;
use std::time::Duration;

pub struct SmartSwitch {
    pub current_state: AtomicBool,
    pub set_state: AtomicBool,
}

impl SmartSwitch {
    pub async fn poll_state(&mut self) {
        println!("kokot");
        sleep(Duration::from_secs(10)).await;
    }
}