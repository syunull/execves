use std::ffi::CStr;

use nix::{unistd::execve, Error};

pub fn execves<P, T>(path: P, arguments: &[T], environment: &[T], secrets: &[T]) -> Result<(), Error>
where
    P: AsRef<CStr>,
    T: AsRef<CStr> + Clone,
{
    execve(path.as_ref(), arguments, &[environment, secrets].concat())?;
    Ok(())
}

pub mod execves;
