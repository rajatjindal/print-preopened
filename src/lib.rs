#[allow(warnings)]
mod bindings;

use crate::bindings::exports::wasi::cli::run::Guest;
use crate::bindings::wasi::filesystem::preopens;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        preopens::get_directories()
            .iter()
            .for_each(|e| println!("{}", e.1));
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
