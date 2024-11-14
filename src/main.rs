#![no_std]
#![no_main]

use waveshare_rp2040_zero as board;

use board::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
	crate::unimplemented!();
}
