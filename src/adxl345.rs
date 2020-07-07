#![allow(dead_code)]  //removes some warnings for the user

//use std::error::Error; //Might add in future but is useless for now

use rppal::i2c::I2c;
//Dump of all the addresses and giving them the same name
//as is in the documentation for the ADXL345
const ADXL_ADD: u16 = 0x53;
//Theese addresess can be referd as COMANDS as they tell the ADXL what
//the user wants to do.
const DEVID: u8 =0;
const THRESH_TAP: u8 =29;
const OFSX: u8 =30;
const OFSY: u8 =31;
const OFSZ: u8 =32;
const DUR: u8 =33;
const LATENT: u8 =34;
const WINDOW: u8 =35;
const THRESH_ACT: u8 =36;
const THRESH_INACT: u8 =37;
const TIME_INACT: u8 =38;
const ACT_INACT_CTL: u8 =39;
const THRESH_FF: u8 =40;
const TIME_FF: u8 =41;
const TAP_AXES: u8 =42;
const ACT_TAP_STATUS: u8 =43;
const BW_RATE: u8 =44;
const POWER_CTL: u8 =45;
const INT_ENABLE: u8 =46;
const INT_MAP: u8 =47;
const INT_SOURCE: u8 =48;
const DATA_FORMAT: u8 =49;
const DATAX0: u8 =50;
const DATAX1: u8 =51;
const DATAY0: u8 =52;
const DATAY1: u8 =53;
const DATAZ0: u8 =54;
const DATAZ1: u8 =55;
const FIFO_CTL: u8 =56;
const FIFO_STATUS: u8 =57;

pub struct Adxl {
    adxl: I2c,          //this is the I2c channel from the RPPAL lib
                        //Not made public for good reasons
    pub id: u8,         //The ID from the accel, not needed but good to see the conection
    pub power_status: u8, //powerstatus, 0 is sleep and 8 is go, might upgrade to a enum
    pub offsets: [u8;3],         //X Y Z offsets, used for calibration
    pub raw_data: [u8;6],       //raw data from the accelerometer
}
//This part contains all functions for the ACCELEROMETER

impl Adxl {
    // Creates a empty struct to allow usage and starts the i2c channel
    //adding strt into this fails for it cant create a new self while working
    //with the current self, basicly keep anything with a refrence to self out of
    //this function
    pub fn new()-> Self{
        let mut _adxl = I2c::new().expect("I2c init failed");   //Starts a new i2c communication
        _adxl.set_slave_address(ADXL_ADD).expect("SETTING SLAVE FAILED"); //Sets the addres ass ADXL_ADD
        //NOTE might add code to allow switching of addresses
        Self{
            //Null values for all except for the i2c channel
            adxl: _adxl,
            id: 0,
            power_status: 0,
            offsets: [0u8;3],
            raw_data: [0u8;6],
        }
    }
    //Simply gets the defult data, so the user can begin
    pub fn start(&mut self){
        self.id = self.get_id(); //gets the id and saves it
        //simple check, the id value should never be 0 or the get_id failed
        if self.id == 0 {println!("Reading the id return 0, this should not happen")}
        //gets the powerstatus, this value can be anything so no checks
        self.power_status = self.get_power_status();
    }
    //uses the private function _read_cmd to read the current id and returns it
    pub fn get_id(&mut self) -> u8{
        self.id = self._read_cmd(DEVID); //sends 0x00 as read command saved as DEVID
        self.id
    }
    //uses the private function _read_cmd to read the current powerstatus and returns it
    pub fn get_power_status(&mut self) -> u8{
        self.power_status = self._read_cmd(POWER_CTL);
        self.power_status
    }
    //uses the private function _write_cmd to read the current id and returns it
    pub fn set_power_status(&self,cmd:u8)->(){
        self._write_cmd(POWER_CTL,cmd);
        let cmd2 = self._read_cmd(POWER_CTL);
        if cmd2 != cmd {println!("POWERCTL, read and write mismatch")}
    }
    //uses the block read function from RPPAL to get 6 values of data from the accelerometer
    //The block read command reads from address DATAX0 to DATAX0 + length(self.raw_data) -1
    //returns it to the struct
    pub fn get_data_raw(&mut self){
        self.adxl.block_read(DATAX0,&mut self.raw_data).expect("READING RAW DATA FAILED");
    }

    //uses the block read function from RPPAL to get 3 values of data from the accelerometer
    //The block read command reads from address OFSX to OFSX + length(self.offsets) -1
    //returns it to the struct
    pub fn get_offsets(&mut self){
        self.adxl.block_read(OFSX,&mut self.offsets).expect("READING OFFSETS FAILED");
    }
    //uses the block write function from RPPAL to set 3 values on the accelerometer
    //The block write command writes from address OFSX to OFSX + length(self.offsets) -1
    // a check can be forced by doing get offsets and comparing, how ever this slows down
    //the code so it is made up to the user
    pub fn set_offsets(&mut self, mut buffer:[u8;3]){
        self.adxl.block_write(OFSX,&mut buffer).expect("WRITING OFFSETS FAILED");
    }
    //Private function that reads of one register nr cmd and returns it as u8
    fn _read_cmd(&self,cmd:u8) ->u8{
        let mut buffer = [0u8;1]; //buffer of length one to get only 1 value out
        self.adxl.block_read(cmd , &mut buffer).expect("Failure in Read CMD");
        buffer[0]
    }
    //Private function that writes to one register nr cmd and gives it the value data
    fn _write_cmd(&self,cmd:u8,data:u8){
        let mut buffer = [0u8;1];//buffer of length 1 to only get 1 value out
        buffer[0]=data;//value passed into buffer
        self.adxl.block_write(cmd ,&mut buffer).expect("Failure in write CMD");
    }
}
