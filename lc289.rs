use std::ops::Index;

pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        // 0b00 dead -> dead
        // 0b10 dead -> live
        // 0b01 live -> dead
        // 0b11 live -> live

        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let rows = board.len();
        let columns = board.index(0).len();

        for row in 0..rows {
            for column in 0..columns {
                let live_neighbors = Self::get_live_neighbors(&board, row as i32, column as i32);
                if board[row][column] == 1 {
                    board[row][column] = match live_neighbors {
                        2 => 0b11,
                        3 => 0b11,
                        _ => 0b01,
                    }
                } else {
                    board[row][column] = match live_neighbors {
                        3 => 0b10,
                        _ => 0b00,
                    }
                }
            }
        }

        for row in board {
            for num in row {
                *num >>= 1;
            }
        }
    }

    fn get_live_neighbors(board: &Vec<Vec<i32>>, row: i32, column: i32) -> i32 {
        let mut live_number = 0;
        let position_list = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (x, y) in position_list {
            let x = row + x;
            let y = column + y;

            if x < 0 || x >= board.len() as i32 || y < 0 || y >= board.index(0).len() as i32 {
                continue;
            }

            live_number += board[x as usize][y as usize] & 0b1;
        }
        live_number
    }
}
