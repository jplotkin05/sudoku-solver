use std::fs;
fn main() {
    //sudoku board is read in and processed into a 2D vector for processing
    let mut board = get_board("tests/puzzle_0.txt".to_string());
    //a before and after of the board is printed
    println!("Unsolved Board:");
    display_board(&board);
    solve(&mut board);
    println!("Solved Board:");
    display_board(&board);
}

fn solve(board:&mut Vec<Vec<i32>>) -> bool{
    let loc = unsolved_cell(board);
    if loc == (-1,-1){
        return true
    }
    
    
    for candidate_value in 1..10{
        if valid_entry(candidate_value, board, loc){
            board[loc.0 as usize][loc.1 as usize] = candidate_value;
            if solve(board){
                return true
            }
        }
        board[loc.0 as usize][loc.1 as usize] = 0;
    }
    return false
}

fn valid_entry(num:i32,board:&Vec<Vec<i32>>,location:(i32,i32)) -> bool{
    //check row
    let row_index = location.0 as usize;
    let col_index = location.1 as usize;
    for i in 0..9{
        if board[row_index][i] == num && i!=col_index{
            return false
        }
    }
    
    //check col
    for j in 0..9{
        if board[j][col_index] == num && j!= row_index{
            return false
        }
    }

    //check box
    let y_base = (location.0 / 3)  *3;
    let x_base = (location.1 / 3)  *3;

    for r in y_base..y_base+3{

        for c in x_base..x_base+3{
            if board[r as usize][c as usize] == num && (r,c)!=location{
                return false
            }
        }
    }

   return true
}


fn unsolved_cell(board:&Vec<Vec<i32>>) ->(i32,i32) {
    for r in 0..9{
        for c in 0..9{
            if board[r][c] == 0{
                return (r as i32, c as i32)
            }
        }
    }
    (-1,-1)
}
fn display_board(board:&Vec<Vec<i32>>){
    for (i,r) in board.iter().enumerate(){
        if i % 3 == 0 && i!=0{
            println!("- - - - - - - - - - -");
        }
        for (j,v) in r.iter().enumerate(){
            if j % 3 == 0{
                print!("|");
            }
                print!("{v} ");
            
        }
        println!("");
    }
}
fn get_board(path:String) -> Vec<Vec<i32>>{
    let mut sudoku_board: Vec<Vec<i32>> = Vec::new();
    let contents  = fs::read_to_string(path)
    .expect("Expected file!");

    for item in contents.lines(){
        let mut fmt_row: Vec<i32> = Vec::new();
        
        for characters in item.lines().flat_map(|l|l.chars()){
            if characters !='|' && characters!=' ' && characters!='-'{
                if characters == '.'{
                    fmt_row.push(0);
                }else{
                    if let Some(int_val) = characters.to_digit(10){
                        fmt_row.push(int_val.try_into().unwrap());
                    }
                }
            }
        }
        if !fmt_row.is_empty(){
            sudoku_board.push(fmt_row);
        }
        
    }
    sudoku_board
}



    #[test]
    fn test_simple_puzzle(){
        let mut board = get_board("tests/puzzle_0.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_0_sol.txt".to_string()));
    }
    #[test]
    fn test_intermediate_puzzle(){
        let mut board = get_board("tests/puzzle_1.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_1_sol.txt".to_string()));
    }
    #[test]
    fn test_expert_puzzle_1(){
        let mut board = get_board("tests/puzzle_2.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_2_sol.txt".to_string()));
    }

    #[test]
    fn test_expert_puzzle_2(){
        let mut board = get_board("tests/puzzle_3.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_3_sol.txt".to_string()));
    }
    #[test]
    fn test_expert_puzzle_3(){
        let mut board = get_board("tests/puzzle_4.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_4_sol.txt".to_string()));
    }

    #[test]
    fn test_expert_puzzle_4(){
        let mut board = get_board("tests/puzzle_5.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_5_sol.txt".to_string()));
    }

    #[test]
    fn test_expert_puzzle_5(){
        let mut board = get_board("tests/puzzle_6.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_6_sol.txt".to_string()));
    }
    #[test]
    fn test_expert_puzzle_7(){
        let mut board = get_board("tests/puzzle_7.txt".to_string());
        solve(&mut board);
        assert_eq!(board,get_board("tests/puzzle_7_sol.txt".to_string()));
    }
