#![no_std]
#![no_main]
use core::fmt;
use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};

// ADC
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig, InterruptHandler as AdcInterruptHandler};
// GPIO
use embassy_rp::gpio::Pull;
//use core::panic::PanicInfo;
use embassy_executor::Spawner;

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use log::info;

// time
use embassy_time::Delay;

//LCD
use embassy_rp::i2c::{I2c, Config};
use ag_lcd::{ Lines,LcdDisplay};
use port_expander::dev::pcf8574::Pcf8574;
use panic_halt as _;

const DISPLAY_FREQ: u32 = 200_000;

bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

// The task used by the serial port driver over USB
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {

    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Initiate SDA and SCL pins
    let sda = peripherals.PIN_4;
    let scl = peripherals.PIN_5;
    
    // Initiate Delay
    let delay = Delay;
    let mut config = Config::default();
    config.frequency = DISPLAY_FREQ;

    // Initiate I2C
    let i2c = I2c::new_blocking(peripherals.I2C0, scl, sda, config.clone());
    let mut i2c_expander = Pcf8574::new(i2c, true, true, true);

    // Initiate LCD
    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
    .with_reliable_init(10000)
    .with_lines(Lines::TwoLines)
    .build();

    // Write to LCD

    lcd.set_character(0u8,[
        0b00000,
        0b00100,
        0b01110,
        0b11111,
        0b11111,
        0b11111,
        0b01110,
        0b00100
    ]);

    lcd.set_character(1u8,[
        0b00000,
        0b00000,
        0b00000,
        0b11011,
        0b11111,
        0b11111,
        0b01110,
        0b00100
    ]);

    lcd.set_character(2u8,[
        0b00000,
        0b00000,
        0b00000,
        0b00100,
        0b01110,
        0b11111,
        0b11111,
        0b11011
    ]);

    lcd.set_character(3u8,[
        0b00000,
        0b00100,
        0b01110,
        0b01110,
        0b10101,
        0b11111,
        0b00100,
        0b00100
    ]);

    lcd.home();
    lcd.write(0u8);
    lcd.cursor_on();

    embassy_time::Timer::after_secs(2).await;

    lcd.blink_on();

    lcd.set_position(3,0);

    lcd.write(1u8);

    lcd.set_position(2,0);

    lcd.write(2u8);

    lcd.set_position(1,0);

    lcd.write(3u8);

    lcd.set_position(0,1);


    

    

    loop {
      info!("Working");
      embassy_time::Timer::after_secs(10).await;
    }


}



//print in textbox  in the window

// print whenever

//2 textboxes to write hwnever we want, we press the 1sst button it prins n! , if press the 2nd witn ehaver vals, it should print n^m --vals can be diff at any moment , be carfeul(read continuously), take svasl from

// NO TIN TERMINAL!!!!!!!!!!!!!!!!!!!
