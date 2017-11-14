extern crate rand;

pub mod msgsystem;
pub mod components;
pub mod world;

pub fn rng_range(start: i32, end: i32) -> i32 {
    let range = (end - start) as u32;
    let offset = start as u32;
    ((rand::random::<u32>() % range) + offset) as i32
}

