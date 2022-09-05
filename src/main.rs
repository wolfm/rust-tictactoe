use std::io::{Write, stdout, Stdout, stdin};

use termion::clear;
use termion::cursor::{self, DetectCursorPos};
use termion::raw::{ IntoRawMode, };
use termion::input::TermRead;
use termion::event::Key;

use rand::Rng;

fn main() {
    let mut board = [[' '; 3]; 3];

    // Enter raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Clear and move cursor to top left corner
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(1,1)).unwrap();
    
    loop {
        write_board(&stdout, &board).unwrap();
        match player_turn(&stdout, &mut board) {
            Err(()) => {break;},
            Ok(()) => {}
        };
        write_board(&stdout, &board).unwrap();
        match check_win(&board) {
            Some(winner) => {
                if winner == 'c' {
                    write!(stdout, "{}Cats game!", cursor::Goto(1,6)).unwrap();
                }
                else {
                    write!(stdout, "{}The winner is '{}'", cursor::Goto(1,6), winner).unwrap();
                }
                break;
            }
            None => {}
        }
        
        ai_turn(&mut board);
        write_board(&stdout, &board).unwrap();
        match check_win(&board) {
            Some(winner) => {
                write!(stdout, "{}The winner is '{}'", cursor::Goto(1,6), winner).unwrap();
                break;
            }
            None => {}
        }
    }

    write!(stdout, "{}", cursor::Goto(1,7)).unwrap();
    stdout.flush().unwrap();
}

fn ai_turn(board: &mut [[char; 3]; 3]) {

    let mut rng = rand::thread_rng();
    loop {
        
        let x = rng.gen_range(0..3);
        let y = rng.gen_range(0..3);
        
        if board[y][x] == ' ' {
            board[y][x] = 'O';
            return;
        }

    }
}

fn player_turn(mut stdout: &Stdout, board: &mut [[char; 3]; 3]) -> Result<(), ()> {    
    loop {
        // Clear the current line.
        // write!(stdout, "{}", termion::cursor::Goto(1, 6)).unwrap();
        stdout.flush().unwrap();

        let c = stdin().keys().next().unwrap();
        match c.unwrap() {
            // Exit.
            Key::Char('q') => return Err(()),
            Key::Ctrl(c)   => {
                if c == 'c' { return Err(()) };
            },
            Key::Left      => {
                
                write!(stdout, "{}", cursor::Left(2)).unwrap();
            },
            Key::Right     => {
                if stdout.cursor_pos().unwrap().0 < 5 {
                    write!(stdout, "{}", cursor::Right(2)).unwrap();
                }
            },
            Key::Up        => {
                write!(stdout, "{}", cursor::Up(2)).unwrap();
            },
            Key::Down      => {
                if stdout.cursor_pos().unwrap().1 < 5 {
                    write!(stdout, "{}", cursor::Down(2)).unwrap();
                }
            },
            Key::Char('\n')=> {
                let (col, row) = stdout.cursor_pos().unwrap();
                board[((col-1)/2) as usize][((row-1)/2) as usize] = 'X';
                break;
            }
            _              => {},
        }
    }
    stdout.flush().unwrap();
    Ok(())
}

fn write_board(mut stdout: &Stdout, board: &[[char; 3]; 3]) -> std::io::Result<()> {
    write!(stdout, "{}{}", cursor::Save, cursor::Goto(1,1))?;
    stdout.write_all(format!("{}|{}|{}\r\n", board[0][0], board[1][0], board[2][0]).as_bytes())?;
    stdout.write_all(b"-+-+-\r\n")?;
    stdout.write_all(format!("{}|{}|{}\r\n", board[0][1], board[1][1], board[2][1]).as_bytes())?;
    stdout.write_all(b"-+-+-\r\n")?;
    stdout.write_all(format!("{}|{}|{}", board[0][2], board[1][2], board[2][2]).as_bytes())?;
    stdout.flush()?;
    write!(stdout, "{}", cursor::Restore)?;
    Ok(())
}

fn check_win(board: &[[char; 3]; 3]) -> Option<char> {
    // Check each row
    for i in 0..3 {
        let target = board[0][i];
        if target == ' ' {
            continue;
        }
        if target == board[1][i] && target == board[2][i] {
            return Some(target);
        }
    }

    // Check each col
    for i in 0..3 {
        let target = board[i][0];
        if target == ' ' {
            continue;
        }
        if target == board[i][1] && target == board[i][2] {
            return Some(target);
        }
    }

    // Check first diag
    {
        let target = board[0][0];
        if target != ' ' && target == board[1][1] && target == board[2][2] {
            return Some(target);
        }
    }

    // Check second diag
    {
        let target = board[0][2];
        if target != ' ' && target == board[1][1] && target == board[2][0] {
            return Some(target);
        }
    }

    // Check for cats
    for i in 0..3 {
        for j in 0..3 {
            if board[j][i] == ' ' {
                return None;
            }
        }
    }
    return Some('c');
}