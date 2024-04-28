#![no_std]
#![no_main]


mod Components;
use Components::SuitsRanks::{Suit, Rank};
use Components::player::Player;
use Components::card::Card;
use Components::create_card::card_init;
use Components::round::{Round,Outcome};

use core::fmt;
use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;
use embassy_time::Delay;

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};

// ADC
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig, InterruptHandler as AdcInterruptHandler};
// GPIO
use embassy_rp::gpio::Pull;

//LCD
use embassy_rp::i2c::{I2c, Config};
use ag_lcd::{ Lines,LcdDisplay};
use port_expander::dev::pcf8574::Pcf8574;

const DISPLAY_FREQ: u32 = 200_000; // display freq of the lcd

// TODO 2: Bind the ADC_IRQ_FIFO interrupt (be careful with the import names)
bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => InterruptHandler<USB>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
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

    // Pins for I2C communication
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


    ////////////////////////////////////////////////////// TESTS ////////////////////////////////////////////////////////


    let bet = 50;

    let mut player = Player::new();
    let mut dealer = Player::new();
    let mut round = Round::new(bet);


    let mut player_card_0 = card_init("Two Club",&round);
    let mut dealer_card_0 = card_init("Ace Heart",&round);
    let mut player_card_1 = card_init("Ace Club",&round);
    let mut dealer_card_1 = card_init("Ace Diamond",&round);


    // keep what is above, just modify

    
    dealer.add_card(&dealer_card_0, &mut round);

    if dealer.score == 1 && dealer.ace_score == 11
    {
        lcd.print("dealer: 1/11");
        embassy_time::Timer::after_secs(5).await;
        lcd.clear();
    }

    dealer.add_card(&dealer_card_1, &mut round);


    if dealer.ace_score == 12 && dealer.score == 2
    {
        lcd.print("dealer: 2/12");
        embassy_time::Timer::after_secs(5).await;
        lcd.clear();
    }


    if let Some(round_card) = round.cards[0] 
    {
        if round_card == dealer_card_0 
        {
            lcd.print("1st is Two Clubs");
            embassy_time::Timer::after_secs(5).await;
            lcd.clear();
        }
    } else {}




    // TODO 0: CREATE INITIAL DEAL "LOOP"
    // TODO 1: CREATE GAME LOOP (maybe do the bet with the potentiometer if done quick)
    // TODO 2: HANDLE GOING OVER 21
    // TODO 3: HANDLE BLACKJACK
    // TODO 4: INSURANCE
    // TODO 5: Choice function => HIT, STAND, DOUBLE, SPLIT -- maybe use  push buttons until WASM is done
    // TODO 6: HANDLE OUTCOMES / RESETS
    // the rest after scanner config is done.

    
    


    // embassy_time::Timer::after_secs(5).await;

    // lcd.clear();



    //set the cursor to (0,0) --  MUST DO AT THE END OF THE ROUND

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    loop{}


}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
