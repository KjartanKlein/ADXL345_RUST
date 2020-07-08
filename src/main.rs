use std::thread;
use std::time::Duration;
mod adxl345;
use crate::adxl345::Adxl;
fn main() {

    let mut accel = Adxl::new();
    accel.start();
    accel.get_offsets();
    accel.get_power_status();
    println!("accelerometer has been kick started");
    println!("ID: {} ;PS: {} ;Offsets ({}, {}, {})",accel.id, accel.power_status, accel.offsets[0],accel.offsets[1],accel.offsets[2]);
    while accel.get_power_status() != 8 {
        println!("Power status is wrong writing 8");
        accel.set_power_status(8);
        thread::sleep(Duration::from_millis(100));
    }
    println!("Starting mesurements");
    loop{
        accel.get_data_raw();
        println!("GOT [ {:?} ]",accel.raw_data);
        thread::sleep(Duration::from_millis(200));
    }
}
