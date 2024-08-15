mod state;
mod ui;

use crate::state::{Dir, State};
use crate::ui::ui;

use std::{
    error::Error,
    io::{self, Stdout},
    process::{Command, Stdio},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let res = run(&mut terminal, State::new());
    restore_terminal(&mut terminal)?;

    if let Ok(Some(key)) = res {
        let cmd = [String::from("pass"), String::from("-c"), key];

        Command::new(&cmd[0])
            .args(&cmd[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
    }

    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut state: State,
) -> io::Result<Option<String>> {
    state.load_keys();

    loop {
        terminal.draw(|f| ui(f, &mut state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.move_index(Dir::Down);
                    }
                    KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.move_index(Dir::Up);
                    }
                    KeyCode::Up => {
                        state.move_index(Dir::Up);
                    }
                    KeyCode::Down => {
                        state.move_index(Dir::Down);
                    }
                    KeyCode::Char(to_insert) => {
                        state.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        state.delete_char();
                    }
                    KeyCode::Enter => {
                        if let Some(i) = state.list_state.selected() {
                            return Ok(Some(state.filtered_keys[i].clone()));
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}
