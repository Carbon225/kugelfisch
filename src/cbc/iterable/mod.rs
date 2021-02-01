use super::{Block, Algo};

mod encryptor;
mod decryptor;

pub use encryptor::*;
pub use decryptor::*;

pub trait BlockIterable<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>
{
    fn encrypt(self, algo: &'a F) -> EncryptorIter<'a, T, F, I>;
    fn decrypt(self, algo: &'a F) -> DecryptorIter<'a, T, F, I>;
}

impl<'a, T, F, I> BlockIterable<'a, T, F, I> for I
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>
{
    fn encrypt(self, algo: &'a F) -> EncryptorIter<'a, T, F, I> {
        EncryptorIter::new(self, algo)
    }

    fn decrypt(self, algo: &'a F) -> DecryptorIter<'a, T, F, I> {
        DecryptorIter::new(self, algo)
    }
}
