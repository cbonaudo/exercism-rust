pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    if len > digits.len() {
        return vec![];
    }

    let mut series = Vec::new();
    let upper_bound = digits.len() - len;

    for i in 0..=upper_bound {
        let mut iter = digits.chars();

        if i > 0 {
            iter.nth(i - 1);
        }

        series.push(
            iter.take(len).collect()
        );
    }

    series
}
