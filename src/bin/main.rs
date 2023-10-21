use std::fs::File;

use execves::Execves;
use nix::Error;

fn main() -> Result<(), Error> {
    let file = File::open("execves.yaml").unwrap();
    let exec = Execves::from_reader(file);
    exec.call();
    Ok(())
}
