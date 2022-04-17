//use std::env;
use std::time::{Duration, Instant};
use std::fs;
use std::thread;


const RADIX: u32 = 10;

fn main() {
    let test_grid = vec!(1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9,
                         1,2,3,4,5,6,7,8,9);
    print_sudoku(&test_grid);
    let mut test_2 = vec![0;81];
    for i in 0..81 {
        test_2[i] = test_grid[transpose_index(i)];
    }
    print_sudoku(&test_2);
    let mut test_3 =  vec![0;81];
    for i in 0..81 {
        test_3[i] = test_2[transpose_index(i)];
    }
    print_sudoku(&test_3);
    let filename = "hard_sudokus.txt";
    //let filename = "all_17_clue_sudokus.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let sudoku_strings =  contents.split_whitespace();
    let sudoku_strings = sudoku_strings.collect::<Vec<&str>>();
    let mut sudoku_puzzles: Vec<Vec<u32>> = Vec::new();

    for s in sudoku_strings {
        let sudoku_puzzle: Vec<u32> = s.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
        sudoku_puzzles.push(sudoku_puzzle);
    }
    let now = Instant::now();
    let mut index = 0;
    let number_of_sudokus = sudoku_puzzles.len();
    //let mut (sudoku_puzzles1, sudoku_puzzles2):(Vec<Vec<u32>>) = sudoku_puzzles.split_at(number_of_sudokus/2);
    let sudokus_per_thread = number_of_sudokus/8;
    let mut sudokus1 = sudoku_puzzles[0..sudokus_per_thread].to_vec();
    let mut sudokus2 = sudoku_puzzles[sudokus_per_thread..2*sudokus_per_thread].to_vec();
    let mut sudokus3 = sudoku_puzzles[2*sudokus_per_thread..3*sudokus_per_thread].to_vec();
    let mut sudokus4 = sudoku_puzzles[3*sudokus_per_thread..4*sudokus_per_thread-1].to_vec();
    let mut sudokus5 = sudoku_puzzles[4*sudokus_per_thread..5*sudokus_per_thread-1].to_vec();
    let mut sudokus6 = sudoku_puzzles[5*sudokus_per_thread..6*sudokus_per_thread-1].to_vec();
    let mut sudokus7 = sudoku_puzzles[6*sudokus_per_thread..7*sudokus_per_thread-1].to_vec();
    let mut sudokus8 = sudoku_puzzles[7*sudokus_per_thread..8*sudokus_per_thread-1].to_vec();

    let handle1 = thread::spawn( move || {
        let mut sudokus = sudokus1;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i);
        }
    });


    let handle2 = thread::spawn( move || {
        let mut sudokus = sudokus2;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+1250);
        }
    });

    let handle3 = thread::spawn( move || {
        let mut sudokus = sudokus3;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+2500);
        }
    });

    let handle4 = thread::spawn( move || {
        let mut sudokus = sudokus4;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+3750);
        }
    });
    let handle5 = thread::spawn( move || {
        let mut sudokus = sudokus5;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+5000);
        }
    });
    let handle6 = thread::spawn( move || {
        let mut sudokus = sudokus6;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+6750);
        }
    });
    let handle7 = thread::spawn( move || {
        let mut sudokus = sudokus7;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+7500);
        }
    });
    let handle8 = thread::spawn( move || {
        let mut sudokus = sudokus8;
        let number_of_sudokus = sudokus.len();
        for i in 1..number_of_sudokus{
            solve_sudoku(&mut sudokus[i], i+8750);
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    handle5.join().unwrap();
    handle6.join().unwrap();
    handle7.join().unwrap(); // infinite loop 
    handle8.join().unwrap();



    /*
    for s in 0..number_of_sudokus {
        solve_sudoku(&mut sudoku_puzzles[s], s);
        if index == 500 {
            break
        }
        index += 1;
    }
    */
    let time_taken = now.elapsed().as_nanos() as f64;
    println!("It took {} seconds", (time_taken / 1000000000 as f64));
}

fn solve_sudoku(sudoku: &mut Vec<u32>, s: usize) {
    println!("The index is: {}",s);
    print_sudoku(&sudoku);

    //checking for naked singels
    check_for_singles(sudoku);

    //checking for hidden singles
    check_for_hidden_singles(sudoku);

    //backtracking
    backtraking(sudoku);
    println!("modified sudoku: ");
    print_sudoku(&sudoku);
}

fn check_sudoku(s: &Vec<u32>, index: usize) -> bool {
    let mut index = index;
    let mut correct = true;

    fn check_row(row: &[u32]) -> bool{
        //for every number there is added 1 to the array using the value as index
        //if any index has more than 2 it means that the row has 2 of the same numbers and thus is invalid
        let mut output = true;
        let mut check:[u32;10] = [0; 10];
        for i in row {
            let j = *i as usize;
            check[j] += 1;
        }
        for i in 1..10 {
            if check[i] > 1 {
                output = false;
                break;
            }
        }
        output
    }
    if index == 81 {
        index = 80;
    }

    
    if correct != false {
        let i = index / 9;
        correct = check_row(&s[i*9..(i+1)*9]);  
    }
    if correct != false {
        let i = index %9;
        let column_as_row = [s[i], s[i+9], s[i+18], s[i +27], s[i+36], s[i+45], s[i + 54], s[i + 63], s[i+72]];
        correct = check_row(&column_as_row);
    }
    if correct != false {
        let i = index;
        let j =  block_to_row(i);
        let block_as_row = [s[j[0]], s[j[1]], s[j[2]], s[j[3]], s[j[4]], s[j[5]], s[j[6]], s[j[7]], s[j[8]]];
        correct = check_row(&block_as_row);
    }
    correct
}
fn check_for_hidden_singles(sudoku: &mut Vec<u32>) {
    let mut numbers_found = 0;
    let mut numbers_changed = true;
    while numbers_changed {
        numbers_changed = false;
        let mut notes = possible_numbers(&sudoku);
        //check_for_doubles(&mut notes);
        //TODO OPTIMIZATION convert columns and blocks to rows before loop
        let mut column_notes = notes.clone();
        let mut block_notes = notes.clone();
        for i in 0..81 {
            column_notes[i] = notes[transpose_index(i)].clone();
            block_notes[i] = notes[block_to_row_index(i)].clone();
        }

        for index in 0..81 {
            if sudoku[index] != 0 {
                continue
            }
            let mut digits_found = [0;10];
            let i = index / 9;
            let row = &notes[i*9..(i+1)*9];
            let column = &column_notes[i*9..(i+1)*9];
            let block = &block_notes[i*9..(i+1)*9];

            for j in 0..9 {
                if j != i  {
                    for n in &row[j] {
                        let n = *n as usize;
                        digits_found[n] += 1;
                    }
                    for n in &column[j] {
                        let n = *n as usize;
                        digits_found[n] += 1;
                    }
                    for n in &block[j] {
                        let n = *n as usize;
                        digits_found[n] += 1;
                    }
                }
            }
            for n in &notes[index] {
                if n == &0 {
                    continue
                }
                let n = *n as usize;
                if  digits_found[n] != 0 {
                    continue
                }
                sudoku[index] = n as u32;
                numbers_found += 1;
                numbers_changed = true;
            }
        }


    }
    //println!("Numbers found {}", numbers_found);
}

fn check_for_singles(sudoku: &mut Vec<u32>) {
    let mut numbers_changed = true;
    while numbers_changed {
        numbers_changed = false;
        let mut notes = possible_numbers(&sudoku);
        check_for_doubles(&mut notes);
        for i in 0..81 {  
            if sudoku[i] != 0 {
                continue;
            } 

            if notes[i].len() == 2 && notes[i][0] == 0 {
                sudoku[i] = notes[i][1];
                numbers_changed = true;
            } 
        }
    }
}

fn check_for_doubles(notes: &mut Vec<Vec<u32>>) -> bool{

    fn update_notes(not: &mut Vec<Vec<u32>>) -> bool {
        let mut changed = false;
        let mut index = 0;
        for i in 0..9 {
            let mut indexes:Vec<usize> = vec!();
            for j in 0..9 {
                index = i*9+j;
                if not[index].len() == 3 {
                    indexes.push(index);
                }
            }
            for j in 0..indexes.len() {
                for k in 0..indexes.len() {
                    if j != k {
                        if not[indexes[j]][1] == not[indexes[k]][1] && not[indexes[j]][2] == not[indexes[k]][2] {
                            let number1 = not[indexes[k]][1];
                            let number2 = not[indexes[k]][2];
                            for l in 0..9 {
                                if i*9 + l != indexes[j] && i*9 + l != indexes[k] {
                                    for m in 0..not[i*9 + l].len(){
                                        if not[i*9 + l][m] == number1 || not[i*9 + l][m] == number2{
                                            not[i*9 + l][m] = 0;
                                            changed = true;
                                        }
                                    }
                                }
                            }
                            
                        }
                        //if double break and update notes
                        //continue until notes don't change and then check for singles
                    }
                }
            }
        }
        changed
    }
    let mut updated = false;
    updated = update_notes(notes); 
    let mut column_notes = notes.clone();
    for i in 0..81 {
        column_notes[i] = notes[transpose_index(i)].clone();
    }
    updated = update_notes(&mut column_notes);
    for i in 0..81 {
        notes[i] = column_notes[transpose_index(i)].clone();
    }
    let mut block_notes = notes.clone();
    for i in 0..81 {
        block_notes[i]  = notes[block_to_row_index(i)].clone();
    }
    updated = update_notes(&mut block_notes);
    for i in 0..81 {
        notes[i]  = block_notes[row_to_block_index(i)].clone();
    }

    for i in 0..81 {
        notes[i].sort();
        notes[i].dedup();
    }
    updated
}

fn block_to_row(index: usize) -> Vec<usize> {        
    match index {
        0  | 1  | 2  | 9  | 10 | 11 | 18 | 19 | 20 => vec!(0, 1, 2, 9, 10, 11, 18, 19, 20),
        3  | 4  | 5  | 12 | 13 | 14 | 21 | 22 | 23 => vec!(3, 4, 5, 12, 13, 14, 21, 22, 23),
        6  | 7  | 8  | 15 | 16 | 17 | 24 | 25 | 26 => vec!(6, 7, 8, 15, 16, 17, 24, 25, 26),
        27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => vec!(27, 28, 29, 36, 37, 38, 45, 46, 47),
        30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => vec!(30, 31, 32, 39, 40, 41, 48, 49, 50),
        33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => vec!(33, 34, 35, 42, 43, 44, 51, 52, 53),
        54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => vec!(54, 55, 56, 63, 64, 65, 72, 73, 74),
        57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => vec!(57, 58, 59, 66, 67, 68, 75, 76, 77),
        60 | 61 | 62 | 69 | 70 | 71 | 78 | 79 | 80 => vec!(60, 61, 62, 69, 70, 71, 78, 79, 80),
        _ => vec!(9,8,7,6,5,4,3,2,1)
    }
}



fn backtraking(sudoku: &mut Vec<u32>) -> bool{
    let mut notes = possible_numbers(&sudoku);
    let mut changed = true;
    while changed {
        changed = check_for_doubles(&mut notes);
    }
    //print_sudoku(sudoku);
    fn backtraking_recursive(sudoku: &mut Vec<u32>, notes: &Vec<Vec<u32>>, index: usize) -> bool {
        let mut solved:bool = false;
        if index == 81 && check_sudoku(&sudoku, index) {
            solved = true;
        }
        else {
            if sudoku[index] != 0 && !solved {
                solved = backtraking_recursive(sudoku, notes, index + 1);
            }
            else if !solved {
                for n in &notes[index] {
                    if n != &0 && !solved {
                        sudoku[index] = *n;
                        //print_sudoku(sudoku);
                        if check_sudoku(&sudoku, index) {
                            solved = backtraking_recursive(sudoku, notes, index + 1);
                        }
                    }
                }
                if !solved {
                    sudoku[index] = 0;
                }
            }
        }
        solved
    }
    backtraking_recursive(sudoku, &notes, 0)
}

fn possible_numbers(sudoku: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut notes:Vec<Vec<u32>> = vec![vec![1,2,3,4,5,6,7,8,9]; 81];
    for i in 0..81 {
        if sudoku[i] != 0 {
            for j in 0..9{
                notes[i][j] = 0;
            }
        } else {
            let row_index = i / 9;
            let column_index = i % 9;
            
            //go trough rows columns and blocks and determine all possible numbers for each cell
            for j in 0..9 {
                // go trough each row and eliminate every number from notes 
                if j != column_index {
                    let check_index = row_index * 9 + j;
                    if notes[i].contains(&sudoku[check_index]) {
                        if sudoku[check_index] != 0 {
                            //println!("here and number is {} and index is {}", sudoku[check_index], check_index);
                            let k = (sudoku[check_index]-1) as usize;
                            notes[i][k] = 0;
                            //println!("{:?}", notes[80]);
                        }
                        
                    }
                }
                
                // go trough each column and eliminate every number from notes
                if j != row_index { 
                    let check_index = column_index + j*9;
                    if notes[i].contains(&sudoku[check_index]) {
                        if sudoku[check_index] != 0 {
                            let k = (sudoku[check_index] - 1) as usize;
                            notes[i][k] = 0;
                        }
                    }
                }
                //println!("{:?}", notes[80]);
            }
            let block_indexes = match i {
                0  | 1  | 2  | 9  | 10 | 11 | 18 | 19 | 20 => vec!(0, 1, 2, 9, 10, 11, 18, 19, 20),
                3  | 4  | 5  | 12 | 13 | 14 | 21 | 22 | 23 => vec!(3, 4, 5, 12, 13, 14, 21, 22, 23),
                6  | 7  | 8  | 15 | 16 | 17 | 24 | 25 | 26 => vec!(6, 7, 8, 15, 16, 17, 24, 25, 26),
                27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => vec!(27, 28, 29, 36, 37, 38, 45, 46, 47),
                30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => vec!(30, 31, 32, 39, 40, 41, 48, 49, 50),
                33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => vec!(33, 34, 35, 42, 43, 44, 51, 52, 53),
                54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => vec!(54, 55, 56, 63, 64, 65, 72, 73, 74),
                57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => vec!(57, 58, 59, 66, 67, 68, 75, 76, 77),
                60 | 61 | 62 | 69 | 70 | 71 | 78 | 79 | 80 => vec!(60, 61, 62, 69, 70, 71, 78, 79, 80),
                _ => vec!(9,8,7,6,5,4,3,2,1)
            };
            for j in block_indexes {
                if notes[i].contains(&sudoku[j]) {
                    if sudoku[j] != 0 {
                        let k = (sudoku[j] - 1) as usize;
                        notes[i][k] = 0;
                    }
                }
            }
        }
    }
    for i in 0..81 {
        notes[i].sort();
        notes[i].dedup();
    }
    //println!("{:?}",notes);

    notes

}


fn print_sudoku(sudoku: &Vec<u32>) {
    println!();
    let mut index = 1;
    for digit in sudoku {
        let digit_char = std::char::from_digit(*digit, RADIX);
        let mut sudoku_char = if digit_char == Some('0') {
            Some('_')
        } else {
            digit_char
        };
        let digit_to_print = sudoku_char.get_or_insert('x');
        print!(" {} ", digit_to_print);
        if index%3 == 0 && index%9 != 0 && index % 27 != 0 {
            print!("|")
        }
        else if index%9 == 0 && index % 27 != 0{
            print!("\n")
        }
        else if index% 27 == 0 && index != 81{
            print!("\n --------------------------- \n")
        }
        index += 1;
    }
    println!();
}

fn transpose_index(index: usize) -> usize {
    match index {
        0 => 0,
        1 => 9,
        2 => 18,
        3 => 27,
        4 => 36,
        5 => 45,
        6 => 54,
        7 => 63,
        8 => 72,
        9 => 1,
        10 => 10,
        11 => 19,
        12 => 28,
        13 => 37,
        14 => 46,
        15 => 55,
        16 => 64,
        17 => 73,
        18 => 2,
        19 => 11,
        20 => 20,
        21 => 29,
        22 => 38,
        23 => 47,
        24 => 56,
        25 => 65,
        26 => 74,
        27 => 3,
        28 => 12,
        29 => 21,
        30 => 30,
        31 => 39,
        32 => 48,
        33 => 57,
        34 => 66,
        35 => 75,
        36 => 4,
        37 => 13,
        38 => 22,
        39 => 31,
        40 => 40,
        41 => 49,
        42 => 58,
        43 => 67,
        44 => 76,
        45 => 5,
        46 => 14,
        47 => 23,
        48 => 32,
        49 => 41,
        50 => 50,
        51 => 59,
        52 => 68,
        53 => 77,
        54 => 6,
        55 => 15,
        56 => 24,
        57 => 33,
        58 => 42,
        59 => 51,
        60 => 60,
        61 => 69,
        62 => 78,
        63 => 7,
        64 => 16,
        65 => 25,
        66 => 34,
        67 => 43,
        68 => 52,
        69 => 61,
        70 => 70,
        71 => 79,
        72 => 8,
        73 => 17,
        74 => 26,
        75 => 35,
        76 => 44,
        77 => 53,
        78 => 62,
        79 => 71,
        80 => 80,
        _  => 0,
    }
}
fn block_to_row_index(index:usize) -> usize {
    match index {
        0  => 0,
        1  => 1,
        2  => 2,
        3  => 9,
        4  => 10,
        5  => 11,
        6  => 18,
        7  => 19,
        8  => 20,
        9  => 3,
        10 => 4,
        11 => 5,
        12 => 12,
        13 => 13,
        14 => 14,
        15 => 21,
        16 => 22,
        17 => 23,
        18 => 6,
        19 => 7,
        20 => 8,
        21 => 15,
        22 => 16,
        23 => 17,
        24 => 24,
        25 => 25,
        26 => 26,
        27 => 27,
        28 => 28,
        29 => 29,
        30 => 36,
        31 => 37,
        32 => 38,
        33 => 45,
        34 => 46,
        35 => 47,
        36 => 30,
        37 => 31,
        38 => 32,
        39 => 39,
        40 => 40,
        41 => 41,
        42 => 48,
        43 => 49,
        44 => 50,
        45 => 33,
        46 => 34,
        47 => 35,
        48 => 42,
        49 => 43,
        50 => 44,
        51 => 51,
        52 => 52,
        53 => 53,
        54 => 54,
        55 => 55,
        56 => 56,
        57 => 63,
        58 => 64,
        59 => 65,
        60 => 72,
        61 => 73,
        62 => 74,
        63 => 57,
        64 => 58,
        65 => 59,
        66 => 66,
        67 => 67,
        68 => 68,
        69 => 75,
        70 => 76,
        71 => 77,
        72 => 60,
        73 => 61,
        74 => 62,
        75 => 69,
        76 => 70,
        77 => 71,
        78 => 78,
        79 => 79,
        80 => 80,
        _  => 0,
    }

}
fn row_to_block_index(index:usize) -> usize {
    match index {
        0 => 0 ,
        1 => 1 ,
        2 => 2 ,
        9 => 3 ,
        10 => 4 ,
        11 => 5 ,
        18 => 6 ,
        19 => 7 ,
        20 => 8 ,
        3 => 9 ,
        4 => 10,
        5 => 11,
        12 => 12,
        13 => 13,
        14 => 14,
        21 => 15,
        22 => 16,
        23 => 17,
        6 => 18,
        7 => 19,
        8 => 20,
        15 => 21,
        16 => 22,
        17 => 23,
        24 => 24,
        25 => 25,
        26 => 26,
        27 => 27,
        28 => 28,
        29 => 29,
        36 => 30,
        37 => 31,
        38 => 32,
        45 => 33,
        46 => 34,
        47 => 35,
        30 => 36,
        31 => 37,
        32 => 38,
        39 => 39,
        40 => 40,
        41 => 41,
        48 => 42,
        49 => 43,
        50 => 44,
        33 => 45,
        34 => 46,
        35 => 47,
        42 => 48,
        43 => 49,
        44 => 50,
        51 => 51,
        52 => 52,
        53 => 53,
        54 => 54,
        55 => 55,
        56 => 56,
        63 => 57,
        64 => 58,
        65 => 59,
        72 => 60,
        73 => 61,
        74 => 62,
        57 => 63,
        58 => 64,
        59 => 65,
        66 => 66,
        67 => 67,
        68 => 68,
        75 => 69,
        76 => 70,
        77 => 71,
        60 => 72,
        61 => 73,
        62 => 74,
        69 => 75,
        70 => 76,
        71 => 77,
        78 => 78,
        79 => 79,
        80 => 80,
        _ => 0 ,
    }

}

