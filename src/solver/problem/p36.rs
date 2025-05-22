pub struct P36;

impl P36 {

    pub fn solve_v1(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let mut row: HashSet<char> = HashSet::new();
        let mut col: HashSet<char> = HashSet::new();
        let mut boxy: HashSet<char> = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                let r = board[i][j];
                let c = board[j][i];
                let b = board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3];

                if r != '.' {
                    if !row.insert(r) {
                        return false;
                    }
                }
                if c != '.' {
                    if !col.insert(c) {
                        return false;
                    }
                }
                if b != '.' {
                    if !boxy.insert(b) {
                        return false;
                    }
                }
            }
            row.clear();
            col.clear();
            boxy.clear();
        }
        true
    }

    pub fn solve_v2(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        
        let mut values: HashSet<char> = HashSet::new();
        
        // returns false if any row contains two of any
        for row in board.iter() {
            for value in row {
                if *value == '.' {continue;}
                if values.contains(&value) {return false;}
                else {values.insert(*value);}
            }
            values.clear();
        }
        
        // returns false if any column contains two of any
        for (col, _) in board[0].iter().enumerate() {
            for (row, _) in board.iter().enumerate() {
                if board[row][col] == '.' {continue;}
                if values.contains(&board[row][col]) {return false;}
                else {values.insert(board[row][col]);}
            }
            values.clear();
        }
        
        // loop through each big square
        for big_row in 0..3 as usize {
            for big_col in 0..3 as usize {
                
                // loop through each small square in the big square
                for small_row in 0..3 as usize {
                    for small_col in 0..3 as usize {
                        
                        // calculate true coords on the board
                        let true_row = big_row * 3 + small_row;
                        let true_col = big_col * 3 + small_col;
                        
                        if board[true_row][true_col] == '.' {continue;}
                        
                        // check if values repeat
                        if values.contains(&board[true_row][true_col]) { return false; }
                        else {values.insert(board[true_row][true_col]);}
                    }
                }
                values.clear();
            }
        }
        // if nothing fails, return true
        true
    }

    pub fn solve_v3(board: Vec<Vec<char>>) -> bool {
        //Cells   (0-8)
        //Rows    (9-17)
        //Columns (18-26)
        
        let mut sets: [u16; 27] = [0; 27];
        
        for (i, row) in board.into_iter().enumerate(){
            for (j, c) in row.into_iter().enumerate(){
                match c {
                    '.' => continue,
                    _  => {
                        let shift = 1 << c.to_digit(10).unwrap() - 1;
                        let cell = (3 * (i / 3)) + (j / 3);
                        
                        if (sets[cell] & shift) == shift || 
                        (sets[j + 9] & shift) == shift || 
                        (sets[i + 18] & shift) == shift{
                            return false;
                        }else{
                            sets[cell] += shift;
                            sets[j + 9] += shift;
                            sets[i + 18] += shift;
                        }
                    }
                }
            }
        }
        true
    }
    
    pub fn solve_v4(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut rows = vec![HashSet::new(); 9];
        let mut cols = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let cell = board[i][j];
                if cell == '.' {
                    continue;
                }

                if !rows[i].insert(cell) {
                    return false;
                }

                if !cols[j].insert(cell) {
                    return false;
                }

                let box_index = (i / 3) * 3 + (j / 3);
                if !boxes[box_index].insert(cell) {
                    return false;
                }
            }
        }
        
        true
    }

}
