extern crate genetic;
extern crate time;

#[cfg(test)]
mod tests {

    use time::PreciseTime;    
    use genetic::*;

    #[test]
    fn test_8_queens() {
        let start = PreciseTime::now();
        let gene_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#";

        let wrapped_display = |candidate: &String| {display_8_queens(&candidate, gene_set, start);};
        let wrapped_get_fitness = |candidate: &String| -> usize {get_8_queens_fitness(&candidate, gene_set)};
    
        let best = genetic::get_best(wrapped_get_fitness, wrapped_display, 8, 8*8*8*8, gene_set);
        println!("Total time: {}", start.to(PreciseTime::now()));
        let best_fitness = get_8_queens_fitness(&best, gene_set);
        assert_eq!(best_fitness,8*8*8*8);
    }
    
    fn display_8_queens(candidate: &String, gene_set: &str, start: PreciseTime) {
        let now = PreciseTime::now();
        let elapsed = start.to(now);
        let board:[[char; 8]; 8] = get_board(candidate, gene_set);
        for i in 0..8 {
            let mut row = "".to_string();
            for j in 0..8 {
                row.push(board[i][j]);
                row.push(' ');
            }
            println!("{}", row);
        }
        
        println!("{}\t{}\t{}", candidate, get_8_queens_fitness(&candidate, gene_set), elapsed);
    }
    
    fn get_8_queens_fitness(candidate: &String, gene_set: &str) -> usize {
        let board = get_board(candidate, gene_set);
        
        // count rows with 1 queen
        let indexes: Vec<i32> = (0..8).collect();
        let mut correct_queens_in_row = 0; 
        for i in 0..8 {
            let row_count = indexes.iter()
                .cloned()
                .map(|col| board[i][col as usize])
                .filter(|ch|'Q' == *ch)
                .count();
            if row_count == 1 {
                correct_queens_in_row = correct_queens_in_row + 1;
            }
        }
        let mut correct_queens_in_column = 0; 
        for i in 0..8 {
            let column_count = indexes.iter()
                .cloned()
                .map(|row| board[row as usize][i])
                .filter(|ch|'Q' == *ch)
                .count();
            if column_count == 1 {
                correct_queens_in_column = correct_queens_in_column + 1;
            }
        }

        let mut correct_queens_in_northeast_diagonal = 0; 
        for i in 0..15 {
            let diag = Diagonal {row:i,col:0,row_offset:-1,col_offset:1};
            let diagonal_count = diag
                .take_while(|point|point.row >= 0 && point.col >= 0)
                .skip_while(|point|point.row >=8 || point.col >=8)
                .take_while(|point|point.col < 8)
                .map(|point| board[point.row as usize][point.col as usize])
                .filter(|ch| 'Q' == *ch)
                .count();
            if diagonal_count == 1 {
                correct_queens_in_northeast_diagonal = correct_queens_in_northeast_diagonal + 1;
            }
        }    

        let mut correct_queens_in_southeast_diagonal = 0; 
        for i in -8..8 {
            let diag = Diagonal {row:i,col:0,row_offset:1,col_offset:1};
            let diagonal_count = diag
                .skip_while(|point|point.row < 0 || point.col < 0)
                .take_while(|point|point.col < 8 && point.row < 8)
                .map(|point| board[point.row as usize][point.col as usize])
                .filter(|ch| 'Q' == *ch)
                .count();
            if diagonal_count == 1 {
                correct_queens_in_southeast_diagonal = correct_queens_in_southeast_diagonal + 1;
            }
        }    
 
        (if correct_queens_in_row == 0 { 1 } else { correct_queens_in_row })
        * (if correct_queens_in_column == 0 { 1 } else { correct_queens_in_column })
        * (if correct_queens_in_northeast_diagonal == 0 { 1 } else { correct_queens_in_northeast_diagonal })
        * (if correct_queens_in_southeast_diagonal == 0 { 1 } else { correct_queens_in_southeast_diagonal })
    }

    fn get_board(candidate: &String, gene_set: &str) -> [[char; 8]; 8] {
        let mut board:[[char; 8]; 8] = [['.'; 8]; 8];
        
        for point in candidate.chars().map(|c| to_point(c, gene_set)) {
            board[point.row as usize][point.col as usize] = 'Q';
        }
        board
    }

    fn to_point(gene: char, gene_set: &str) -> Point {
        let location = gene_set.find(gene);
        assert_eq!(location.is_some(), true);
        let index = location.unwrap() as i32;
        let row = index / 8i32;
        let column = index % 8i32;
        return Point{row: row, col: column};
    }
    
    struct Point {
        row: i32,
        col: i32
    }
    
    struct Diagonal {
        row: i32,
        col: i32,
        row_offset: i32,
        col_offset: i32
    }

    impl Iterator for Diagonal {
        type Item = Point;
        fn next(&mut self) -> Option<Point> {
            let prev_row = self.row;
            let prev_col = self.col;
            self.row = prev_row + self.row_offset;
            self.col = prev_col + self.col_offset;
            
            // 'Some' is always returned, this is an infinite value generator
            Some(Point{row:prev_row,col:prev_col})
        }
    }

    #[test]
    fn test_diagonal_iterator_first_value_returned_should_be_the_start_state() {
        let mut diag = Diagonal {row:5,col:6,row_offset:1,col_offset:1};
        let first = diag.next();
        assert_eq!(true,first.is_some());
        let point = first.unwrap() as Point;
        assert_eq!(point.row,5);
        assert_eq!(point.col,6);
    }
    
    #[test]
    fn test_diagonal_iterator_second_value_returned_should_be_with_offsets_added_to_start_state() {
        let diag = Diagonal {row:5,col:4,row_offset:1,col_offset:-1};
        let first = diag.skip(1).next();
        assert_eq!(true,first.is_some());
        let point = first.unwrap() as Point;
        assert_eq!(point.row,6);
        assert_eq!(point.col,3);
    }
}
