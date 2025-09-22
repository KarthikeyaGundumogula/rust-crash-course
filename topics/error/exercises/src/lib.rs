use std::fmt::Error;

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    match y{
        0 => Err(MathError::DivByZero),
        _ => Ok(x/y)
    }
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    let x = v.get(i);
    match x{
        Some(x)=> *x,
        None=>default_val
    }
}
