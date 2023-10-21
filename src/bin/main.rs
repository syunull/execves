use std::fs::File;

use execves::execves::Execves;
use nix::Error;

fn main() -> Result<(), Error> {
    let file = File::open("execves.yaml").unwrap();
    let wrapper = Execves::from_reader(file);
    wrapper.call();
    Ok(())
}
