fn even(nb: u64) -> u64 {
    nb / 2
}

fn odd(nb: u64) -> u64 {
    3 * nb + 1
}

pub fn collatz(n: u64) -> Option<u64> {
    let mut ans = n;
    if n == 0 {
        return None;
    }

    let mut i = 0;
    while ans > 1 {
        if ans % 2 == 0 {
            ans = even(ans);
        } else {
            ans = odd(ans);
        } 
        i = i + 1;
    }
    Some(i)
}
