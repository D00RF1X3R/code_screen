use crate::code_screen::start_code;

fn main() {
    let _ = start_code();
}


mod code_screen {
    use std::io::Write;
    use std::io::{self};
    use std::thread::sleep;
    use std::time::Duration;
    use std::vec;

    use crossterm::{ExecutableCommand, execute};
    use crossterm::event::{Event, KeyCode, poll, read};
    use crossterm::style::{Color, Print, SetForegroundColor, StyledContent, Stylize};
    use crossterm::terminal::{self, Clear, disable_raw_mode, enable_raw_mode};
    use rand::seq::IteratorRandom;

    struct ColoredChar {
    
    }


    pub fn start_code() -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        let mut rng = rand::rng();

        enable_raw_mode()?;

        let symbols = "01 sdfa   ".chars(); 
        let colors = "rgb".chars();
        loop {  
            if poll(Duration::from_millis(5))? {
                match read()? {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Esc => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    Event::FocusGained => todo!(),
                    Event::FocusLost => todo!(),
                    Event::Mouse(_mouse_event) => todo!(),
                    Event::Paste(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                }
            }

            let (cols, rows) = terminal::size()?;
            stdout.execute(Clear(terminal::ClearType::All))?;
            

            for _row in 0..rows {
                let mut line = "".to_string();
                for _col in 0..cols {
                    line += &symbols.clone().choose(&mut rng).unwrap().to_string();
                }
                let color_char = &colors.clone().choose(&mut rng).unwrap().to_string();
                let mut color = Color::White;
                if color_char == "r" {
                    color = Color::Red;
                } else if color_char == "g" {
                    color = Color::Green
                } else if color_char == "b" {
                    color = Color::Blue
                }
                execute!(
                    stdout,
                    SetForegroundColor(color),
                    Print(line.clone())
                )?;

                //write!(stdout, "\r\n{}", line)?;      
            }
            // надо сделать структуру char разных цветов, она будет собираться и выводиться посимвольно.
            
            stdout.flush()?;

        }
        



        disable_raw_mode()?;
        return Ok(());
    }





}



