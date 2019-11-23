//! This shows how an application can write on stderr
//! instead of stdout, thus making it possible to
//! the command API instead of the "old style" direct
//! unbuffered API.
//!
//! This particular example is only suited to Unix
//! for now.

use std::io::{stderr, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event,
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    Output, Result,
};
use crossterm::event::{KeyCode, Event, KeyEvent};

const TEXT: &str = r#"
This screen is ran on stderr.
And when you hit enter, it prints on stdout.
This makes it possible to run an application and choose what will
be sent to any application calling yours.

For example, assuming you build this example with

    cargo build --bin stderr

and then you run it with

    cd "$(target/debug/stderr)"

what the application prints on stdout is used as argument to cd.

Try it out.

Hit any key to quit this screen:

1 will print `..`
2 will print `/`
3 will print `~`
Any other key will print this text (so that you may copy-paste)
"#;

fn run_app<W>(write: &mut W) -> Result<char>
where
    W: Write,
{
    queue!(
        write,
        EnterAlternateScreen, // enter alternate screen
        Hide                  // hide the cursor
    )?;

    let mut y = 1;
    for line in TEXT.split('\n') {
        queue!(write, MoveTo(1, y), Output(line.to_string()))?;
        y += 1;
    }

    write.flush()?;

    let _raw = RawScreen::into_raw_mode()?;
    let user_char = read_char()?; // we wait for the user to hit a key
    queue!(write, Show, LeaveAlternateScreen)?; // restore the cursor and leave the alternate screen

    write.flush()?;
    Ok('f')
}

pub fn read_char() -> Result<char> {
    loop {
        if let Event::Key(KeyEvent {code: KeyCode::Char(c), .. }) = event::read()?
        {
            return Ok(c);
        }
    }
}

pub fn read_line() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code: code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => {
                break;
            },
            KeyCode::Char(c) => {
                line.push(c);
            },
            _ => {}
        }
    }

    return Ok(line);
}

// cargo run --example stderr
fn main() {
    match run_app(&mut stderr()).unwrap() {
        '1' => print!(".."),
        '2' => print!("/"),
        '3' => print!("~"),
        _ => println!("{}", TEXT),
    }
}