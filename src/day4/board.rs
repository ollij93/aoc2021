// Bingo board implementation for day4 of Advent of Code 2021

pub struct BingoBoard(pub [[u32; 5]; 5]);

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for row in self.0 {
            for val in row {
                write!(f, "{:3}", val).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl BingoBoard {
    pub fn eliminate_num(&self, num: u32) -> BingoBoard {
        BingoBoard(self.0.map(|row| row.map(|v| if v == num { 0 } else { v })))
    }
    pub fn is_solved(&self) -> bool {
        self.0.iter().any(|row| *row == [0; 5])
            || (0..5).any(|i| self.0.map(|row| row[i]) == [0; 5])
    }
    pub fn sum(&self) -> u32 {
        self.0.iter().fold(0, |x, row| x + row.iter().sum::<u32>())
    }
}

pub fn parse_board(input: &[String]) -> BingoBoard {
    // Expecting a single empty line then 5 lines with 5 numbers on each
    (0..5).fold(BingoBoard([[0; 5]; 5]), |brd, rowi| {
        let line = &input[rowi + 1];
        let nums: Vec<u32> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        (0..5).fold(brd, |mut brd, ci| {
            brd.0[rowi][ci] = nums[ci];
            brd
        })
    })
}
