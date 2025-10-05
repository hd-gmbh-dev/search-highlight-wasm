mod error;
mod error_impls;

pub use error::Error;
#[inline]
pub fn getrandom(_dest: &mut [u8]) -> Result<(), Error> {
    Ok(())
}