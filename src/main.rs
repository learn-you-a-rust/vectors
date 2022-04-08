fn new_pixel_buffer(rows:usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn main() {
    let mut primes = vec![2,3,5,7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

}
