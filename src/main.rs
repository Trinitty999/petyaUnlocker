#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi_services::*;
use uefi::proto::console::text::*;
use ansi_rgb::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    init_console(system_table.stdout());

    println!("Hello world!");

    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}

pub fn init_console(stdout: &mut Output) {
    let best_mode = stdout.modes().last().unwrap();

    stdout
        .set_mode(best_mode)
        .expect("Failed to change text mode");

    stdout
        .set_color(Color::White, Color::Black)
        .expect("Failed to change console color");

    stdout.clear().expect("Failed to clear screen");
}
