use crate::cbc::{cbc_decrypt, Block, Algo};

pub struct DecryptorIter<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>
{
    algo: &'a F,
    prev: Option<T>,
    input: I,
}

impl<'a, T, F, I> DecryptorIter<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>,
{
    pub fn new(mut input: I, algo: &'a F) -> Self {
        let prev = input.next();
        DecryptorIter { algo, input, prev }
    }
}

impl<'a, T, F, I> Iterator for DecryptorIter<'a, T, F, I>
    where T: Block,
          F: Algo<T>,
          I: Iterator<Item=T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let block = self.input.next()?;
        let p = cbc_decrypt(block, self.prev?, self.algo);
        self.prev = Some(block);
        Some(p)
    }
}
