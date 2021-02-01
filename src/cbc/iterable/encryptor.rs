use crate::cbc::{cbc_encrypt, Block, Algo};

pub struct EncryptorIter<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>
{
    algo: &'a F,
    prev: Option<T>,
    input: I,
}

impl<'a, T, F, I> EncryptorIter<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>
{
    pub fn new(mut input: I, algo: &'a F) -> EncryptorIter<'a, T, F, I> {
        let prev = input.next();
        EncryptorIter { algo, input, prev }
    }
}

impl<'a, T, F, I> Iterator for EncryptorIter<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let block = self.input.next()?;
        self.prev = Some(cbc_encrypt(block, self.prev?, self.algo));
        self.prev
    }
}
