pub const CHUNK_SIZE : usize = 16;

pub const MAX_TEN_POWER : u64 = 10_000_000_000_000_000;

pub const TEN_POWS : [u64; 17] = [
  1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
  1000000000, 10000000000, 100000000000, 1000000000000, 10000000000000, 100000000000000, 1000000000000000, 10_000_000_000_000_000
];



pub mod bytes;