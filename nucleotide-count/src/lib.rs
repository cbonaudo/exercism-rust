use std::collections::HashMap;

static NUC_CHAR: [char; 4] = ['A', 'T', 'G', 'C'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    NUC_CHAR
        .iter()
        .find(|c| **c == nucleotide)
        .ok_or(nucleotide)?;

    dna.chars()
        .map(|f| NUC_CHAR.iter().find(|c| **c == f).ok_or(f))
        .collect::<Result<Vec<&char>, char>>()?;

    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hashmap = HashMap::new();

    NUC_CHAR.iter().try_for_each(|c| -> Result<(), char> {
        hashmap.insert(*c, count(c.clone(), dna)?);
        Ok(())
    })?;

    Ok(hashmap)
}
