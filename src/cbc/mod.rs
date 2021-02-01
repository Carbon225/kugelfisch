#[cfg(test)]
mod tests;

mod slice;
pub use slice::*;

mod processor;
pub use processor::*;

use std::ops::BitXor;

pub trait Block: Copy + BitXor<Output=Self> {}

impl<T> Block for T
    where T: Copy + BitXor<Output=Self>
{}

pub trait Algo<T>: Fn(T) -> T {}

impl<T, F> Algo<T> for F
    where T: Block,
          F: Fn(T) -> T
{}

pub fn cbc_encrypt<T, F>(block: T, prev: T, algo: &F) -> T
    where T: Block, F: Algo<T>
{
    algo(block ^ prev)
}

pub fn cbc_decrypt<T, F>(block: T, prev: T, algo: &F) -> T
    where T: Block, F: Algo<T>
{
    algo(block) ^ prev
}
