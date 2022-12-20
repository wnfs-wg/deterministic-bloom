use deterministic_bloom::{test_utils::Rvg, BloomFilter};

#[test]
fn test_contains() {
    let mut bloom = BloomFilter::<256, 30>::new();
    let mut rvg = Rvg::new();

    let new_val: Vec<u8> = vec![rvg.sample(&(..255u8))];
    bloom.add(&new_val);

    for _ in 1..25 {
        bloom.add(&vec![rvg.sample(&(..255u8))]);
    }

    assert!(bloom.contains(&new_val))
}
