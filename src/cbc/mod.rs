#[cfg(test)]
mod tests;

pub trait Block<T>: Copy + std::ops::BitXor<Output = T> {}
impl<T: Copy + std::ops::BitXor<Output = T>> Block<T> for T {}

pub trait Algo<T>: Fn(T) -> T {}
impl<T: Block<T>, F: Fn(T) -> T> Algo<T> for F {}

pub fn cbc_encrypt<T, F>(block: T, prev: T, algo: &F) -> T
    where T: Block<T>, F: Algo<T> {
    algo(block ^ prev)
}

pub fn cbc_decrypt<T, F>(block: T, prev: T, algo: &F) -> T
    where T: Block<T>, F: Algo<T> {
    algo(block) ^ prev
}

#[allow(dead_code)]
pub fn encrypt_blocks<T, F>(blocks: &mut [T], algo: &F)
    where T: Block<T>, F: Algo<T> {
    for i in 1..blocks.len() {
        blocks[i] = cbc_encrypt(blocks[i], blocks[i - 1], algo);
    }
}

#[allow(dead_code)]
pub fn decrypt_blocks<T, F>(blocks: &mut [T], algo: &F)
    where T: Block<T>, F: Algo<T> {
    for i in (1..blocks.len()).rev() {
        blocks[i] = cbc_decrypt(blocks[i], blocks[i - 1], algo);
    }
}

pub trait BlockProcessor<T> {
    fn next(&mut self, block: T) -> T;
}

pub struct Encryptor<'a, T: Block<T>, F: Algo<T>> {
    algo: &'a F,
    prev: T
}

impl<'a, T: Block<T>, F: Algo<T>> Encryptor<'a, T, F> {
    pub fn new(algo: &'a F, iv: T) -> Self {
        Encryptor { algo, prev: iv }
    }
}

impl<'a, T: Block<T>, F: Algo<T>> BlockProcessor<T> for Encryptor<'a, T, F> {
    fn next(&mut self, block: T) -> T {
        self.prev = cbc_encrypt(block, self.prev, self.algo);
        self.prev
    }
}

pub struct Decryptor<'a, T: Block<T>, F: Algo<T>> {

    algo: &'a F,
    prev: T
}

impl<'a, T: Block<T>, F: Algo<T>> Decryptor<'a, T, F> {
    pub fn new(algo: &'a F, iv: T) -> Self {
        Decryptor { algo, prev: iv }
    }
}

impl<'a, T: Block<T>, F: Algo<T>> BlockProcessor<T> for Decryptor<'a, T, F> {
    fn next(&mut self, block: T) -> T {
        let p = cbc_decrypt(block, self.prev, self.algo);
        self.prev = block;
        p
    }
}
