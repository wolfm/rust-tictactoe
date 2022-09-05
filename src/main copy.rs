use std::io::{Write, stdout, Stdout, stdin, Read};
use std::time::Duration;
use std::thread::sleep;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};

fn main() {
    let mut stdout = stdout();
    let mut board = ['O'; 9];
    let mut game_over = false;

    stdout.execute(cursor::Hide).unwrap();
    stdout.queue(cursor::SavePosition).unwrap();

    while !game_over {
        write_board(&stdout, board);
        make_turn(&stdout, board);
        sleep(Duration::from_secs(1));
        game_over = true;
    }

    stdout.execute(cursor::Show).unwrap();
}

fn make_turn(mut stdout: &Stdout, board: [char; 9]) {
    let mut selected_pos = 0;
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.queue(cursor::Show).unwrap();
    stdout.queue(cursor::EnableBlinking).unwrap();
    stdout.flush();

    match stdin().bytes().next() {
        Event::Key(KeyEvent{ code: KeyCode::Right })
        None => {}
    };

    stdout.queue(cursor::Hide).unwrap();
}

fn write_board(mut stdout: &Stdout, board: [char; 9]) {
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.write_all(format!("{}|{}|{}\n", board[0], board[1], board[2]).as_bytes()).unwrap();
    stdout.write_all(b"-+-+-\n").unwrap();
    stdout.write_all(format!("{}|{}|{}\n", board[3], board[4], board[5]).as_bytes()).unwrap();
    stdout.write_all(b"-+-+-\n").unwrap();
    stdout.write_all(format!("{}|{}|{}", board[6], board[7], board[8]).as_bytes()).unwrap();
    stdout.flush().unwrap();
    // stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
}

