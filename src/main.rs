extern crate serial;
extern crate wiringpi;

use std::env;
use std::io;
use std::time::Duration;

use serial::prelude::*;
use std::io::prelude::*;

use std::boxed::Box;
use std::{thread, time};
use wiringpi::pin::Value::{High, Low};

use std::sync::mpsc::*;

///
/// Pin enumeration for easier use
///
pub enum MotorCommand {
    A_Forward(u16),
    B_Forward(u16),
    A_Back(u16),
    B_Back(u16),
}
pub const MOTOR_A_IN_PIN_0: u16 = 0;
pub const MOTOR_A_IN_PIN_1: u16 = 1;

pub const MOTOR_B_IN_PIN_0: u16 = 2;
pub const MOTOR_B_IN_PIN_1: u16 = 3;

pub const MOTOR_A_PWM_PIN: u16 = 4;
pub const MOTOR_B_PWM_PIN: u16 = 5;

pub fn init_pi(recv: Receiver<MotorCommand>) {
    let pi = wiringpi::setup();

    let In0A = pi.output_pin(MOTOR_A_IN_PIN_0);
    let In1A = pi.output_pin(MOTOR_A_IN_PIN_1);

    let In0B = pi.output_pin(MOTOR_B_IN_PIN_0);
    let In1B = pi.output_pin(MOTOR_B_IN_PIN_1);

    let PWMA = pi.output_pin(MOTOR_A_PWM_PIN);
    let PWMB = pi.output_pin(MOTOR_B_PWM_PIN);

    In0A.digital_write(Low);
    In1A.digital_write(Low);
    In0B.digital_write(Low);
    In1B.digital_write(Low);

    PWMA.digital_write(High);
    PWMB.digital_write(High);
    loop {
        let command = match recv.recv() {
            Ok(command) => command,
            Err(e) => {
                eprintln!{"{}",e}
                return;
            }
        };
        let milis = match command {
            MotorCommand::A_Forward(steps) => {
                In0A.digital_write(High);
                In1A.digital_write(Low);
                steps
            }
            MotorCommand::B_Forward(steps) => {
                In0B.digital_write(High);
                In1B.digital_write(Low);
                steps
            }
            MotorCommand::A_Back(steps) => {
                In0A.digital_write(Low);
                In1A.digital_write(High);
                steps
            }
            MotorCommand::B_Back(steps) => {
                In0B.digital_write(Low);
                In1B.digital_write(High);
                steps
            }
        };
        let duration = Duration::from_millis(milis as u64);
        thread::sleep(duration);

        In0A.digital_write(Low);
        In1A.digital_write(Low);
        In0B.digital_write(Low);
        In1B.digital_write(Low);
    }
}

pub struct DataPort {
    pub serial_port: Box<SerialPort>,
}

fn configure_port<T: SerialPort>(serial_port: &mut T) -> io::Result<()> {
    try!(serial_port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));
    Ok(())
}

fn main() {
    println!("S.H.I.EL.D. server startup."); /*
    let mut serial_port = serial::open("/dev/USB0").unwrap();
    configure_port(&mut serial_port).unwrap();
    let data_port = DataPort {
        serial_port: Box::new(serial_port),
    };*/
    let (send, recv) = channel();
    thread::spawn(move || init_pi(recv));
    thread::sleep(Duration::from_secs(2));
    send.send(MotorCommand::A_Forward(100));
    thread::sleep(Duration::from_secs(2));
}
