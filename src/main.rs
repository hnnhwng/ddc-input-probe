use ddc;
use ddc_macos;
use std::io;

use ddc::Ddc;
use ddc_macos::Monitor;

fn main() {
    loop {
        let monitors = Monitor::enumerate().expect("Could not enumerate external monitors");
        let mut input_line = String::new();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let hex: u16 = u16::from_str_radix(input_line.trim(), 16).expect("Parse error");
        println!("Hex: {:#04x}", hex);

        for mut monitor in monitors {
            println!("Trying {:#04x} on {}", hex, monitor.description());
            let _ = monitor.set_vcp_feature(0x60, hex);
            let _ = monitor.save_current_settings();
        }
    }
}
