pub fn block_align(n:u64, block_size:u64) -> u64 {
    let t = n + block_size - 1;

    return t - (t % block_size);
}