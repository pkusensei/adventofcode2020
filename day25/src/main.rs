const DENOM: u64 = 20201227;

const PUB_KEY1: u64 = 17115212;
const PUB_KEY2: u64 = 3667832;

fn main() {
    let loop1 = find_loop_size(7, PUB_KEY1);
    let loop2 = find_loop_size(7, PUB_KEY2);
    let ek1 = find_encryption_key(PUB_KEY2, loop1);
    let ek2 = find_encryption_key(PUB_KEY1, loop2);
    assert_eq!(ek1, ek2);
    assert_eq!(15467093, ek2);

    println!("ALL DONE!");
}

fn find_loop_size(sub_num: u64, target: u64) -> u64 {
    let mut value = 1;
    let mut count = 0;
    while value != target {
        value *= sub_num;
        value %= DENOM;
        count += 1
    }
    count
}

fn find_encryption_key(sub_num: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= sub_num;
        value %= DENOM;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(8, find_loop_size(7, 5764801));
        assert_eq!(11, find_loop_size(7, 17807724));
    }

    #[test]
    fn test_find_encryption_key() {
        assert_eq!(14897079, find_encryption_key(17807724, 8));
        assert_eq!(14897079, find_encryption_key(5764801, 11));
    }
}
