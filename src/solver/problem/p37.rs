pub struct P37;

impl P37 {
    pub fn solve_v1(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }
    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for n in 1..=9 {
                        let c = std::char::from_digit(n, 10).unwrap();
                        if Self::valid(&board, i, j, c) {
                            board[i][j] = c;
                            if Self::solve(board) {
                                return true;
                            }
                        }
                    }
                    board[i][j] = '.';
                    return false;
                }
            }
        }
        true
    }

    fn valid(board: &Vec<Vec<char>>, i: usize, j: usize, c: char) -> bool {
        let i_base = i / 3 * 3;
        let j_base = j / 3 * 3;
        for offset in 0..9 {
            if board[offset][j] == c {
                return false;
            }
            if board[i][offset] == c {
                return false;
            }
            if board[i_base + offset / 3][j_base + offset % 3] == c {
                return false;
            }
        }
        true
    }

    pub fn solve_v2(board: &mut Vec<Vec<char>>) {
        let mut idx = 0;
        let mut direction = Direction::Right;
        let mut sudoku = Sudoku::new(board);
        while idx < 81 {
            match sudoku.tiles[idx] {
                Tile::Const(_) => match direction {
                    Direction::Right => idx += 1,
                    Direction::Left => idx -= 1,
                },
                Tile::Number(_) | Tile::Empty => match sudoku.next_available_number(idx) {
                    Some(n) => {
                        sudoku.set_tile(idx, Tile::Number(n));
                        direction = Direction::Right;
                        idx += 1;
                    }
                    None => {
                        sudoku.delete_tile(idx);
                        direction = Direction::Left;
                        idx -= 1;
                    }
                },
            }
        }
        sudoku.write_to(board);
    }
    
    pub fn solve_v3(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];
        let mut empty = vec![];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    empty.push((i, j));
                } else {
                    let num = board[i][j] as u8 - b'1';
                    let mask = 1 << num;
                    rows[i] |= mask;
                    cols[j] |= mask;
                    boxes[Self::box_index(i, j)] |= mask;
                }
            }
        }

        Self::solve_board(board, &mut rows, &mut cols, &mut boxes, &mut empty, 0);
    }

    fn solve_board(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
        empty: &mut Vec<(usize, usize)>,
        index: usize,
    ) -> bool {
        if index == empty.len() {
            return true;
        }

        // MCV heuristic: find the cell with the fewest options
        let mut min_opts = 10;
        let mut min_index = index;

        for k in index..empty.len() {
            let (i, j) = empty[k];
            let b = Self::box_index(i, j);
            let used = rows[i] | cols[j] | boxes[b];
            let count = 9 - used.count_ones();

            if count < min_opts {
                min_opts = count;
                min_index = k;
                if count == 1 {
                    break;
                }
            }
        }

        empty.swap(index, min_index);
        let (i, j) = empty[index];
        let b = Self::box_index(i, j);

        for num in 0..9 {
            let mask = 1 << num;
            if (rows[i] & mask) == 0 && (cols[j] & mask) == 0 && (boxes[b] & mask) == 0 {
                rows[i] |= mask;
                cols[j] |= mask;
                boxes[b] |= mask;
                board[i][j] = (b'1' + num as u8) as char;

                if Self::solve_board(board, rows, cols, boxes, empty, index + 1) {
                    return true;
                }

                board[i][j] = '.';
                rows[i] &= !mask;
                cols[j] &= !mask;
                boxes[b] &= !mask;
            }
        }

        false
    }

    fn box_index(i: usize, j: usize) -> usize {
        (i / 3) * 3 + (j / 3)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Number(u8),
    Const(u8),
}

impl Tile {
    pub fn to_u16(&self) -> u16 {
        match self {
            Tile::Empty => 0,
            Tile::Number(n) | Tile::Const(n) => *n as u16,
        }
    }
}

struct Sudoku {
    tiles: [Tile; 81],
    rows: [u16; 9],
    cols: [u16; 9],
    squares: [u16; 9],
}

impl Sudoku {
    fn new(tiles: &Vec<Vec<char>>) -> Self {
        let mut sudoku = Sudoku {
            tiles: [Tile::Empty; 81],
            rows: [0; 9],
            cols: [0; 9],
            squares: [0; 9],
        };
        tiles
            .iter()
            .flat_map(|row| {
                row.iter().map(|tile| match tile {
                    '.' => Tile::Empty,
                    n => Tile::Const((*n as u32 - '0' as u32) as u8),
                })
            })
            .enumerate()
            .for_each(|(i, tile)| sudoku.set_tile(i, tile));
        sudoku
    }

    fn write_to(&self, board: &mut Vec<Vec<char>>) {
        self.tiles
            .iter()
            .enumerate()
            .for_each(|(i, tile)| match tile {
                Tile::Number(n) => {
                    board[i / 9][i % 9] = char::from_u32(*n as u32 + '0' as u32).unwrap()
                }
                _ => (),
            });
    }

    fn set_tile(&mut self, indx: usize, tile: Tile) {
        self.clear(indx);
        self.tiles[indx] = tile;
        let mask = 1 << tile.to_u16();
        self.rows[indx / 9] |= mask;
        self.cols[indx % 9] |= mask;
        self.squares[indx / 27 * 3 + (indx % 9) / 3] |= mask;
    }

    fn delete_tile(&mut self, indx: usize) {
        self.clear(indx);
        self.tiles[indx] = Tile::Empty;
    }

    fn clear(&mut self, indx: usize) {
        let mask = !(1 << self.tiles[indx].to_u16());
        if mask != 0b1111111111111110 {
            self.rows[indx / 9] &= mask;
            self.cols[indx % 9] &= mask;
            self.squares[indx / 27 * 3 + (indx % 9) / 3] &= mask;
        }
    }

    fn next_available_number(&self, indx: usize) -> Option<u8> {
        let available = self.avaliable_numbers(indx) | ((1 << (self.tiles[indx].to_u16() + 1)) - 1);
        match available.trailing_ones() {
            n if n <= 9 => Some(n as u8),
            _ => None,
        }
    }

    fn avaliable_numbers(&self, indx: usize) -> u16 {
        let row = indx / 9;
        let col = indx % 9;
        let square = indx / 27 * 3 + (indx % 9) / 3;
        self.rows[row] | self.cols[col] | self.squares[square]
    }
}
