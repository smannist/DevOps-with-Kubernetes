use std::thread::sleep;
use std::time::Duration;

use jiff::Timestamp;
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();

    loop {
        let ts = Timestamp::now()
            .strftime("%Y-%m-%dT%H:%M:%S%.3fZ");
        println!("{ts}: {id}");
        sleep(Duration::from_secs(5));
    }
}
