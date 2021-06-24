pub struct PascalsTriangle {
    row_nb: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_nb: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows_vec = Vec::new();

        for row_nb in 0..self.row_nb {
            if row_nb == 0 {
                rows_vec.push(vec![1]);
                continue;
            }

            let mut row_vec = Vec::new();
            

            for i in 0..=row_nb {

                let left_hand = get_left_hand(i, &rows_vec[(row_nb - 1) as usize]);
                let right_hand = get_right_hand(i, &rows_vec[(row_nb - 1) as usize], row_nb);
                let sum = left_hand + right_hand;

                row_vec.push(sum);
            }

            rows_vec.push(row_vec);
        }

        rows_vec
    }
}

pub fn get_left_hand(i: u32, prev_row: &Vec<u32>) -> u32 {
    if i == 0 {
        return 0;
    } else {
        return prev_row[(i - 1) as usize];
    }
}

pub fn get_right_hand(i: u32, prev_row: &Vec<u32>, row_nb: u32) -> u32 {
    if i >= row_nb {
        return 0;
    } else {
        return prev_row[i as usize];
    }
}