fn has_corresponding(c: char, temp: &Vec<char>) -> bool {
    if let Some(last_char) = temp.iter().last() {
        *last_char == c
    } else {
        false
    }
}

fn remove_corresponding(c: char, temp: &mut Vec<char>) {
    let index = temp.iter().rev().position(|x| *x == c).unwrap();
    temp.remove(temp.len() - index - 1);
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut temp: Vec<char> = Vec::new();

    for c in string.chars().collect::<Vec<char>>() {
        if c == '[' || c == '{' || c == '(' {
            temp.push(c);
        }

        if c == ']' || c == '}' || c == ')' {
            let corres = match c {
                ']' => '[',
                '}' => '{',
                ')' => '(',
                _ => ' ',
            };

            if has_corresponding(corres, &temp) {
                remove_corresponding(corres, &mut temp);
            } else {
                return false;
            }
        }
    }

    temp.len() == 0 
}
