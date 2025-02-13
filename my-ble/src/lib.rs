#![no_std]

#[derive(defmt::Format)]
pub enum Event {
    Buzz,
    Idle,
}

pub mod ble;
