use serde_json::{from_str, json};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Mine,
    Block,
    Revealed(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Diff {
    position: Position,
    value: Cell,
}

struct ErrTable {
    message: String,
    position: Position,
}

type Board = Vec<Vec<Cell>>;
type Position = (usize, usize);

// struct SolveOutput {
//     result:bool,
//     position: [usize; 2],
//     output:String,
// }

#[wasm_bindgen]
pub fn solve(board: String) -> String {
    let mut _board = parse_board_text(board);
    let valid_chack = is_valid(&_board, true);
    if (&valid_chack).is_err() {
        let err_table = valid_chack.unwrap_err();
        return json!({
            "result": false,
            "position": [
                err_table.position.0,
                err_table.position.1,
            ],
            "output": err_table.message
        })
        .to_string();
    }
    let mut output = vec![];
    let (trimed_board, skipped_position) = skip_check(&_board);
    let solved = _solve(trimed_board.clone());
    for item in &solved {
        let mut _item = vec![];
        for row in item {
            let mut _row = vec![];
            for cell in row {
                _row.push(boardcell_to_number(*cell));
            }
            _item.push(_row);
        }
        output.push(_item);
    }
    let diff = get_diff(&trimed_board, &solved);
    // for skiped in skipped_position {
    //     for (i,item) in diff.clone().iter().enumerate() {
    //         if item.position == skiped {
    //             diff.remove(i as usize);
    //         }
    //     }
    // }
    return json!({
        "result": true,
        "output": diff.iter().map(|item| {
            json!({
                "position": [
                    item.position.0,
                    item.position.1,
                ],
                "value": boardcell_to_number(item.value)
            })
        }).collect::<serde_json::Value>(),
        "position":[-1,-1]
    })
    .to_string();
}

fn skip_check(board: &Board) -> (Board, Vec<Position>) {
    let mut board = board.clone();
    let mut skip_position = vec![];
    for x in 0..board.len() {
        'loop1: for y in 0..board[0].len() {
            let cell = board[x][y];
            if cell == Cell::Block {
                let arround = get_arround_cell(&board, (x, y));
                for arround_cell in arround {
                    if arround_cell != Cell::Block {
                        continue 'loop1;
                    }
                }
                skip_position.push((x, y));
                board[x][y] = Cell::Mine;
            }
        }
    }
    return (board.to_vec(), skip_position);
}

fn get_diff(input_board: &Board, solved_board: &Vec<Board>) -> Vec<Diff> {
    println!("{:?}", solved_board);
    let mut output = vec![];
    for (x, row) in input_board.iter().enumerate() {
        'loop1: for (y, cell) in row.iter().enumerate() {
            if solved_board.len() == 0 {
                continue 'loop1;
            }
            let first_solved_cell = &solved_board[0][x][y];
            if first_solved_cell == cell {
                continue 'loop1;
            }
            for solved_item in solved_board {
                if solved_item[x][y] != *first_solved_cell {
                    continue 'loop1;
                }
            }
            output.push(Diff {
                position: (x, y),
                value: first_solved_cell.clone(),
            });
        }
    }
    return output;
}

fn parse_board_text(board_text: String) -> Board {
    let board: Vec<Vec<usize>> = from_str(board_text.as_str()).unwrap();
    const EMPTY: usize = 0;
    const MINE: usize = 10;
    const BLOCK: usize = 11;
    let mut _board = vec![];
    for row in board {
        let mut _row = vec![];
        for cell in row {
            if cell == MINE {
                _row.push(Cell::Mine);
            } else if cell == BLOCK {
                _row.push(Cell::Block);
            } else if cell == EMPTY {
                _row.push(Cell::Empty);
            } else {
                _row.push(Cell::Revealed(cell));
            }
        }
        _board.push(_row);
    }
    return _board;
}

fn boardcell_to_number(cell: Cell) -> usize {
    match cell {
        Cell::Empty => 0,
        Cell::Mine => 10,
        Cell::Block => 11,
        Cell::Revealed(n) => n,
    }
}

fn _solve(board: Board) -> Vec<Board> {
    let legal_list = _main(board.clone(), get_all_block_place(board));
    // for ls in &legal_list {
    //     println!("{:#?}", format_board(&ls));
    // }
    return legal_list;
}

fn format_board(board: &Board) -> String {
    let mut output = String::new();
    for row in board {
        for cell in row {
            output += &convert_cell_print(*cell);
        }
        output += "\n";
    }
    return output;
}

fn convert_cell_print(cell: Cell) -> String {
    match cell {
        Cell::Empty => "_".to_string(),
        Cell::Mine => "*".to_string(),
        Cell::Block => "X".to_string(),
        Cell::Revealed(num) => num.to_string(),
    }
}

fn _main(board: Board, block_place: Vec<Position>) -> Vec<Board> {
    if block_place.len() == 0 {
        if is_valid(&board, false).is_ok() {
            return vec![board];
        } else {
            return vec![];
        }
    }
    let mut result = vec![];
    let mut board1 = board.clone();
    let mut board2 = board.clone();
    let mut new_block_place = block_place.clone();
    let (block_x, block_y) = new_block_place.pop().unwrap();
    board1[block_x][block_y] = Cell::Empty;
    board2[block_x][block_y] = Cell::Mine;

    let mut output1 = _main(board1, new_block_place.clone());
    let mut output2 = _main(board2, new_block_place);

    result.append(&mut output1);
    result.append(&mut output2);
    result
}

fn is_valid(board: &Board, allow_block: bool) -> Result<(), ErrTable> {
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] == Cell::Mine || board[x][y] == Cell::Empty {
                continue;
            }
            if board[x][y] == Cell::Block {
                if allow_block {
                    continue;
                } else {
                    return Err(ErrTable {
                        message: "invalid board".to_string(),
                        position: (x, y),
                    });
                }
            }
            let mut count = 0;
            // let mut block_count = 0;
            let arround = get_arround_cell(board, (x, y));
            for cell in &arround {
                if cell == &Cell::Mine || cell == &Cell::Block {
                    count += 1;
                }
            }
            if let Cell::Revealed(arround_mine) = board[x][y] {
                // println!("{:?}", arround);
                if (!allow_block && count != arround_mine) || (allow_block && count < arround_mine)
                {
                    return Err(ErrTable {
                        message: "invalid board".to_string(),
                        position: (x, y),
                    });
                }
            } else {
                unreachable!();
            }
        }
    }
    return Ok(());
}

fn get_all_block_place(board: Board) -> Vec<Position> {
    let mut result = vec![];
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] == Cell::Block {
                result.push((x, y));
            }
        }
    }
    return result;
}

#[inline]
fn get_arround_cell(board: &Board, pos: Position) -> Vec<Cell> {
    let mut result = vec![];
    let (x, y) = pos;
    let x = x as isize;
    let y = y as isize;
    for x in x - 1..=x + 1 {
        for y in y - 1..=y + 1 {
            if pos.0 == x as usize && pos.1 == y as usize {
                continue;
            }
            if x < 0 || y < 0 || x >= board.len() as isize || y >= board[0].len() as isize {
                continue;
            }

            result.push(board[x as usize][y as usize]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main() {
        // let board = vec![
        //     vec![Cell::Empty, Cell::Empty, Cell::Empty],
        //     vec![Cell::Empty, Cell::Block, Cell::Revealed(2)],
        //     vec![Cell::Block, Cell::Block, Cell::Revealed(1)],
        //     vec![Cell::Block, Cell::Block, Cell::Empty],
        //     vec![Cell::Revealed(1), Cell::Revealed(2), Cell::Mine],
        // ];
        // _solve(board);
        println!(
            "{:#?}",
            solve(
                "[
            [11,11,11,10],
            [11,11,4,10],
            [10,10,3,2],
            [1,2,10,1]
        ]"
                .to_string()
            )
        );
        // println!("{:#?}", solve(
        //     "[
        //     [11,1],
        //     [1,1]
        // ]"
        //     .to_string(),
        // ));
        println!(
            "{:#?}",
            solve(
                "[
            11,0,0,0,0
            11,3,2,1,0
            11,11,11,11,11]
        ]"
                .to_string(),
            )
        );
    }
}
