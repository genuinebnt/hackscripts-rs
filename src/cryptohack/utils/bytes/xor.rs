pub fn xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, val)| val ^ key[i % key.len()])
        .collect()
}
