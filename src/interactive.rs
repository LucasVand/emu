use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use emulator::Emulator;

pub fn interactive_start(emulator: &mut Emulator) {
    if let Err(e) = enable_raw_mode() {
        eprintln!("Failed to enable raw mode: {}", e);
        emulator.start();
        return;
    }

    loop {
        emulator.cycle();

        if emulator.memory.is_halted() {
            if emulator.verbose >= 1 {
                println!("\rProgram halted");
            }
            break;
        }

        print!("\rPress SPACE to continue (q to quit)...    \r");
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        if let Ok(Event::Key(key_event)) = event::read() {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char(' ') => {
                        continue;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        println!("\rInteractive execution stopped");
                        break;
                    }
                    KeyCode::Esc => {
                        println!("\rInteractive execution stopped");
                        break;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
    println!("\r");
    let _ = disable_raw_mode();
}
