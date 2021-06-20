use rand::thread_rng;
use rand::Rng;
use std::convert::TryInto;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a.try_into().unwrap()) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a.try_into().unwrap()) % p
}
