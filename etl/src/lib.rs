use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut btr = BTreeMap::new();

    h.iter().for_each(|(points, chars)| {
        for c in chars {
            btr.insert((*c).to_ascii_lowercase(), *points);
        }
    });

    btr
}
