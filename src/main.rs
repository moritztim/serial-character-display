#![no_std]
#![no_main]

use waveshare_rp2040_zero as board_support_package;

use board_support_package::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
	crate::unimplemented!();
}
