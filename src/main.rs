mod telemetry;
use std::thread::sleep;
use std::time::Duration;
use crate::telemetry::{get_mmap, update_telemetry};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mmap = get_mmap("/dev/shm/LMU_Data");

    loop {
        let telemetry = update_telemetry(&mmap).unwrap();

        let engine_rpm = telemetry.telemetry.telemetry_info[0].m_engine_rpm;

        println!("Engine RPM: {:?}", engine_rpm);
        // print_debug(&shm_layout);
        sleep(Duration::from_millis(50));
    }
}