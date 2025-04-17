#![feature(const_mut_refs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
extern crate alloc;
pub mod cpu;
use alloc::vec::Vec;
use alloc::string::String;

#[cfg(feature = "std")]
mod tests{
    use super::*;
    use cpu::kind::ATtiny13A;
    use std::{fs, env};
    #[test]
    pub fn main(){
        let args: Vec<String> = env::args().collect();
        let file = fs::read(args[1].clone()).unwrap();
        let file_u16: Vec<u16> = file.chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect();
        let core = cpu::CPU::new(file_u16.as_slice(), ATtiny13A);
    }
}