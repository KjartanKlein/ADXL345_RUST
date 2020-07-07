use std::thread;
use std::time::Duration;
mod adxl345;
use adxl345::Adxl;
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













/*
fn main() -> Result<(), Box<dyn Error>> {
    println!("Figuring out ADXL I2C thing");
    let mut adxl = I2c::new()?;
    adxl.set_slave_address(ADXL_ADD)?;

    let mut po_s = [0u8;1];

    let mut id = [0u8; 1];

    let mut rtap = [0u8;1];
    let mut wtap = [0u8;1];

    //println!("ORG STATES po_s = {:?} ; id = {:?} ; til = {:?}",po_s, id, til);
    adxl.block_read(0x00 as u8, &mut id)?;
    adxl.block_read(45u8, &mut po_s)?;
    println!("{:?}",id[0]);
    println!("{:?}",po_s[0]);
    if po_s[0] != 8 {
        println!("Changing power state to go!");
        let mut cmd = [8u8;1];
        adxl.block_write(45u8,&mut cmd)?;
        thread::sleep(Duration::from_millis(50));
        adxl.block_read(45u8, &mut po_s)?;
        println!("{:?}",po_s[0]);
    }
    adxl.block_read(0x1D as u8, &mut rtap)?;
    adxl.block_write(0x1D as u8, &mut wtap)?;
    println!("{:?}",rtap[0]);

    println!("Turning on the tap");
    let mut cmd = [0b00001111 as u8;1];
    adxl.block_write(0x2A as u8,&mut cmd)?;
    let mut cmd = [0b00000000 as u8;1];
    adxl.block_read(0x2A as u8,&mut cmd)?;
    println!("0x2a set, {:?}",cmd);

    let mut cmd = [0b01000000 as u8;1];
    adxl.block_write(0x2E as u8,&mut cmd)?;
    let mut cmd = [0b00000000 as u8;1];
    adxl.block_read(0x2E as u8,&mut cmd)?;
    println!("0x2e set, {:?}",cmd);

    let mut cmd = [0b00001111 as u8;1];
    adxl.block_write(0x2C as u8,&mut cmd)?;
    let mut cmd = [0b00000000 as u8;1];
    adxl.block_read(0x2C as u8,&mut cmd)?;
    println!("0x2e set, {:?}",cmd);

    let mut cmd = [0b01000000 as u8;1];
    adxl.block_write(0x2F as u8,&mut cmd)?;
    let mut cmd = [0u8;1];
    adxl.block_read(0x2F as u8,&mut cmd)?;
    println!("0x2f set, {:?}",cmd);

    println!("tap is on");
    loop {
        let mut cmd = [0u8;1];
        let mut til = [0u8; 6];
        adxl.block_read(50u8, &mut til)?;
        adxl.block_read(0x30 as u8, &mut cmd)?;
        println!("Cmd is currently : {:?}",cmd[0]);
        thread::sleep(Duration::from_millis(100));
    }
    //Ok(())

}
*/
