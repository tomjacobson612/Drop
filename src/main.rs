#![no_std]
#![no_main]

use cortex_m_rt::entry;
use lsm303agr::Lsm303agr;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        pac::{self, interrupt, TIMER0, TIMER1},
        prelude::*, 
        timer::Timer, 
        twim,
        delay::Delay,
        gpio::Level},
    pac::twim0::frequency::FREQUENCY_A,
};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use critical_section_lock_mut::LockMut;

/// The display is shared by the main program and the
/// interrupt handler.
static DISPLAY: LockMut<Display<TIMER0>> = LockMut::new();

static dot: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
]);

static exclamation: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 9, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 9, 0, 0],
]);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let display = Display::new(board.TIMER0, board.display_pins);
    DISPLAY.init(display);
   
    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER0, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER0);
    }

    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100,
    );

    let mut timer2 = Timer::new(board.TIMER2);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    sensor.set_accel_mode_and_odr(
        &mut timer2,
        lsm303agr::AccelMode::Normal,
        lsm303agr::AccelOutputDataRate::Hz1,
    ).unwrap();

    //sound stuff
    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);

    let mut total: f32 = 0.0;
    let threshold: f32 = 0.75;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let data = sensor.acceleration().unwrap();
        
            x = data.x_mg() as f32 / 1000.0; // Convert mg to g
            y = data.y_mg() as f32 / 1000.0;
            z = data.z_mg() as f32 / 1000.0;
        
            total = x * x + y * y + z * z; // Calculate magnitude squared in g^2
        }
        
            if total < threshold {
                speaker.set_high().unwrap();
                delay.delay_us(500u16);
                speaker.set_low().unwrap();
                delay.delay_us(500u16);
                DISPLAY.with_lock(|display| display.show(&exclamation));
            }
            else {
                DISPLAY.with_lock(|display| display.show(&dot));
            }
        
    }
}

#[interrupt]
fn TIMER0() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
