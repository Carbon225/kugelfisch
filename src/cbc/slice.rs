use super::{Block, Algo, cbc_decrypt, cbc_encrypt};

#[allow(dead_code)]
pub fn encrypt_blocks<T, F>(blocks: &mut [T], algo: &F)
    where T: Block,
          F: Algo<T>
{
    for i in 1..blocks.len() {
        blocks[i] = cbc_encrypt(blocks[i], blocks[i - 1], algo);
    }
}

#[allow(dead_code)]
pub fn decrypt_blocks<T, F>(blocks: &mut [T], algo: &F)
    where T: Block,
          F: Algo<T>
{
    for i in (1..blocks.len()).rev() {
        blocks[i] = cbc_decrypt(blocks[i], blocks[i - 1], algo);
    }
}