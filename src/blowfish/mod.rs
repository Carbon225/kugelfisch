use std::mem::swap;
use std::num::Wrapping;

#[cfg(test)]
mod tests;

mod boxes;

fn f(x: &u32, s: &[[u32; 256]; 4]) -> u32 {
    let d = (x & 0x000000ff) as usize;
    let c = ((x & 0x0000ff00) >> 8) as usize;
    let b = ((x & 0x00ff0000) >> 16) as usize;
    let a = ((x & 0xff000000) >> 24) as usize;

    let s_a = Wrapping(s[0][a]);
    let s_b = Wrapping(s[1][b]);
    let s_c = Wrapping(s[2][c]);
    let s_d = Wrapping(s[3][d]);

    (((s_a + s_b) ^ s_c) + s_d).0
}

fn feistel(x_l: &mut u32, x_r: &mut u32, p: &[u32], s: &[[u32; 256]; 4]) {
    for pi in p[0..p.len() - 2].iter() {
        *x_l ^= pi;
        *x_r ^= f(x_l, s);
        swap(x_l, x_r);
    }

    swap(x_l, x_r);
    *x_r ^= p[p.len() - 2];
    *x_l ^= p[p.len() - 1];
}

// TODO combine with normal version?
fn feistel_rev(x_l: &mut u32, x_r: &mut u32, p: &[u32], s: &[[u32; 256]; 4]) {
    for pi in p[2..p.len()].iter().rev() {
        *x_l ^= pi;
        *x_r ^= f(x_l, s);
        swap(x_l, x_r);
    }

    swap(x_l, x_r);
    *x_r ^= p[1];
    *x_l ^= p[0];
}

pub fn generate_keys(key: &[u32]) -> ([u32; 18], [[u32; 256]; 4]) {
    let mut p: [u32; 18] = boxes::P_ARRAY.clone();
    let mut s: [[u32; 256]; 4] = boxes::S_BOX.clone();

    for i in 0..p.len() {
        p[i] ^= key[i % key.len()];
    }

    let mut x_l = 0;
    let mut x_r = 0;

    for i in (0..p.len()).step_by(2) {
        feistel(&mut x_l, &mut x_r, &p, &s);
        p[i] = x_l;
        p[i + 1] = x_r;
    }

    for i in 0..4 {
        for j in (0..256).step_by(2) {
            feistel(&mut x_l, &mut x_r, &p, &s);
            s[i][j] = x_l;
            s[i][j + 1] = x_r;
        }
    }

    (p, s)
}

fn split64(x: u64) -> (u32, u32) {
    ((x >> 32) as u32, x as u32)
}

fn merge64(x_l: u32, x_r: u32) -> u64 {
    ((x_l as u64) << 32) | (x_r as u64)
}

pub fn encrypt(x: u64, p: &[u32], s: &[[u32; 256]; 4]) -> u64 {
    let (mut x_l, mut x_r) = split64(x);
    feistel(&mut x_l, &mut x_r, p, s);
    merge64(x_l, x_r)
}

pub fn decrypt(x: u64, p: &[u32], s: &[[u32; 256]; 4]) -> u64 {
    let (mut x_l, mut x_r) = split64(x);
    feistel_rev(&mut x_l, &mut x_r, p, s);
    merge64(x_l, x_r)
}
