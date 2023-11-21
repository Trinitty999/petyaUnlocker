#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi_services::*;
use uefi::table::boot::*;
use uefi::proto::console::text::*;
use uefi::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    
    init_console(system_table.stdout());

    println!("Hello world!\n");

    // let mut bootservices = BootSer;

    // let mut stdin = system_table.stdin();

    // read_keyboard_events(&bootservices, &mut stdin);

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

fn read_keyboard_events(boot_services: &mut BootServices, input: &mut Input) -> Result {
    loop {
        // Pause until a keyboard event occurs.
        let mut events = unsafe { [input.wait_for_key_event().unwrap()] };
        boot_services
            .wait_for_event(&mut events)
            .discard_errdata()?;

        let u_key = Char16::try_from('u').unwrap();
        match input.read_key()? {
            // Example of handling a printable key: print a message when
            // the 'u' key is pressed.
            Some(Key::Printable(key)) if key == u_key => {
                println!("the 'u' key was pressed");
            }

            // Example of handling a special key: exit the loop when the
            // escape key is pressed.
            Some(Key::Special(ScanCode::ESCAPE)) => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}