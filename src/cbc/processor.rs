use super::{Block, Algo, cbc_decrypt, cbc_encrypt};

pub trait BlockProcessor<T> {
    fn next(&mut self, block: T) -> T;
}

pub struct Encryptor<'a, T, F>
    where T: Block,
          F: Algo<T>
{
    algo: &'a F,
    prev: T,
}

impl<'a, T, F> Encryptor<'a, T, F>
    where T: Block,
          F: Algo<T>
{
    pub fn new(algo: &'a F, iv: T) -> Self {
        Encryptor { algo, prev: iv }
    }
}

impl<'a, T, F> BlockProcessor<T> for Encryptor<'a, T, F>
    where T: Block,
          F: Algo<T>
{
    fn next(&mut self, block: T) -> T {
        self.prev = cbc_encrypt(block, self.prev, self.algo);
        self.prev
    }
}

pub struct Decryptor<'a, T, F>
    where T: Block,
          F: Algo<T>
{
    algo: &'a F,
    prev: T,
}

impl<'a, T, F> Decryptor<'a, T, F>
    where T: Block,
          F: Algo<T>
{
    pub fn new(algo: &'a F, iv: T) -> Self {
        Decryptor { algo, prev: iv }
    }
}

impl<'a, T, F> BlockProcessor<T> for Decryptor<'a, T, F>
    where T: Block,
          F: Algo<T>
{
    fn next(&mut self, block: T) -> T {
        let p = cbc_decrypt(block, self.prev, self.algo);
        self.prev = block;
        p
    }
}