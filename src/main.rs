use avr_device::atmega328p::Peripherals;

fn main() {
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();
}