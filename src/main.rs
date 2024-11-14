#![no_std]
#![no_main]

use waveshare_rp2040_zero as board;

use board::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
	info!("Program start");

	// Instance of the peripherals access API
	let mut peripherals = board::pac::Peripherals::take().unwrap();

	let mut watchdog = board::hal::Watchdog::new(peripherals.WATCHDOG);

	let clocks = board::hal::clocks::init_clocks_and_plls(
		board::XOSC_CRYSTAL_FREQ,
		peripherals.XOSC,
		peripherals.CLOCKS,
		peripherals.PLL_SYS,
		peripherals.PLL_USB,
		&mut peripherals.RESETS,
		&mut watchdog,
	);

	// Single Cycle Input and Output
	let sio = board::hal::Sio::new(peripherals.SIO);

	let pins = board::Pins::new(
		peripherals.IO_BANK0,
		peripherals.PADS_BANK0,
		sio.gpio_bank0,
		&mut peripherals.RESETS,
	);

	crate::unimplemented!();
}
