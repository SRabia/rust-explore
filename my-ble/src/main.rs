#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_nrf::gpio::Pin;
use embassy_nrf::interrupt::Priority;
use embassy_nrf::peripherals::PWM0;
use embassy_nrf::pwm::SimplePwm;
// global logger
use embassy_nrf as _; // time driver
use panic_probe as _;

use static_cell::StaticCell;

use defmt::info;
use embassy_executor::Spawner;

use my_ble::ble;
use my_ble::Event;

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel, signal::Signal};

static SIGNAL: Signal<ThreadModeRawMutex, Event> = Signal::new();
// static CHANNEL: Channel<ThreadModeRawMutex, Event, 5> = Channel::new();

const TONE: u32 = 1500;
static PWM0: StaticCell<SimplePwm<'static, PWM0>> = StaticCell::new();

#[embassy_executor::task]
async fn buzzer_task(pwm: &'static mut SimplePwm<'static, PWM0>) {
    info!("buzzer task started");
    // Initial PWM configuration

    loop {
        if let Event::Buzz = SIGNAL.wait().await {
            // No need to reconfigure these every time since they're already set
            // pwm.set_duty(0, pwm.max_duty() / 2);
            // pwm.set_period(TONE);
            pwm.enable();
            pwm.set_period(TONE);
            pwm.set_duty(0, pwm.max_duty() / 2);

            embassy_time::Timer::after_millis(80).await;
            pwm.set_duty(0, 0);
            pwm.disable();
        }
    }
}

#[embassy_executor::task]
async fn button(pin: embassy_nrf::gpio::AnyPin) {
    let mut input = embassy_nrf::gpio::Input::new(pin, embassy_nrf::gpio::Pull::None);
    let mut press_count = 0;
    loop {
        input.wait_for_low().await;
        press_count += 1;
        embassy_time::Timer::after_millis(100).await;
        SIGNAL.signal(Event::Buzz);
        input.wait_for_high().await;
        embassy_time::Timer::after_millis(50).await;
    }
}

#[embassy_executor::task]
async fn task_ble(bletask: my_ble::ble::BleTask) {
    bletask.run().await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Application init");

    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    let pwm = PWM0.init(SimplePwm::new_1ch(p.PWM0, p.P0_00));
    let myble = ble::BleTask::new(&SIGNAL);
    myble.run_softdevice(&spawner);

    spawner.spawn(buzzer_task(pwm)).unwrap();
    spawner.spawn(button(p.P0_14.degrade())).unwrap();
    // spawner.spawn(task_ble(myble)).unwrap();
    loop {
        myble.run().await;
    }
}
