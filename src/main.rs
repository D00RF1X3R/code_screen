use std::env;

use crate::code_screen::start_code;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    if args.len() == 1 {
        let _ = start_code(None);
    } else {
        let colors_arg = &args[1];
        let mut colors: Vec<[i32;3]> = vec![];
        let mut built_color: [ i32; 3 ] = [0; 3];
        let mut counter = 0;
        for color in colors_arg.split(',') {
            if counter % 3 == 2 {
                built_color[counter%3] = color.parse::<i32>().expect("Насрано");
                colors.push(built_color.clone());
            } else {
                built_color[counter%3] = color.parse::<i32>().expect("Насрано");
            }
            counter += 1;
        }
        let _ = start_code(Some(colors));
        
    }
    
}


mod code_screen {
    use std::io::Write;
    use std::io::{self};
    use std::time::Duration;

    use crossterm::cursor::{Hide, MoveTo, Show};
    use crossterm::{ExecutableCommand, execute};
    use crossterm::event::{Event, KeyCode, poll, read};
    use crossterm::style::{Print,};
    use crossterm::terminal::{self, Clear, disable_raw_mode, enable_raw_mode};
    use rand::{RngExt};
    use rand::seq::{IndexedRandom, IteratorRandom};

    fn make_colored_string(symbol: char, rgb: [i32; 3]) -> String {
        // "\x1b[38;2;255;255;255m"
        let colored_string: String;
        if symbol != ' '{
            colored_string = format!("\x1b[48;2;1;1;1m\x1b[38;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
        } else {
            colored_string = format!("\x1b[38;2;255;255;255m\x1b[48;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
        }

        return colored_string;

    }

    struct CodeSeuqence { //TODO: Сделать несколько видов секвенсов
        coords: [i32; 2],
        color: [i32; 3],
        line: Vec<char>,
        length: i32

    }
    impl CodeSeuqence {
        pub fn new(col_start:i32, rgb: [i32; 3], length: i32, chars: &str) -> Self {
            let chars_iter = chars.chars();
            let mut line: Vec<char> = vec![];
            let mut rng = rand::rng();
            for _ in 0..length {
                line.push(chars_iter.clone().choose(&mut rng).unwrap());
            }
            let seq = CodeSeuqence {coords: [0, col_start], color: rgb, length, line};

            return seq
        }
        pub fn step(&mut self) {
            self.coords[0] += 1
        }
        pub fn get_top_coord(&self) -> [i32; 2] {
            return self.coords;
        }
        pub fn len(&self) -> i32 {
            return self.length;
        }
        pub fn get_color(&self) -> [i32; 3] {
            return self.color;
        }
        pub fn get_symbol_by_index(&self, i: usize) -> char {
            match self.line.get(i) {
                Some (char ) => {return *char}
                None => {panic!()}
            }
        }
    }
    pub fn get_random_color(colors_vec: &Vec<[i32; 3]>) -> [ i32; 3 ] {
        let mut rng = rand::rng();
        colors_vec.clone().choose(&mut rng).unwrap().clone()
    }
    fn check_seq_on_coords(row: i32, col: i32, sequences: &Vec<CodeSeuqence>) -> Option<&CodeSeuqence> {
        return sequences.iter().find(|f| (row..(row+f.len())).contains(&f.coords[0]) && f.coords[1] == col.into());
    }


    pub fn start_code(colors: Option<Vec<[i32; 3]>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        let mut rng = rand::rng();
        let colors_vec = colors.unwrap_or([[0, 255, 0]].to_vec());
        let mut sequences: Vec<CodeSeuqence> = vec![];

        enable_raw_mode()?;
        stdout.execute(Clear(terminal::ClearType::All))?;
        stdout.execute(Hide)?;

        let symbols = "10"; 
        
        loop {  
            let (cols, rows) = terminal::size()?;
            for _ in  0..rng.random_range(1..6){
                let seq = CodeSeuqence::new(
                    rng.random_range(0..cols).into(),
                    get_random_color(&colors_vec),
                    rng.random_range(1..30),
                    symbols
                );
                
                sequences.push(seq);
            }
            if poll(Duration::from_millis(40))? {
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
                    Event::Resize(_, _) => {continue;},
                }
            }
            stdout.execute(MoveTo(0, 0))?;
            
            for row in 0..rows {
                let mut line = "".to_string();
                for col in 0..cols {
                    let row_i32: i32 = row.into();
                    match check_seq_on_coords(row_i32, col.into(), &sequences) {
                        Some(seq) => {
                            let rgb = seq.get_color();
                            let symbol = seq.get_symbol_by_index((seq.get_top_coord()[0] - row_i32).try_into().unwrap());
                            let colored_string = &make_colored_string(symbol, rgb);
                            line += colored_string;
                        }
                        None => {
                            line += " ";
                        }
                    }
                    
                    


                    //     let rgb = colors_vec.clone().choose(&mut rng).unwrap().clone();
                    //     let symbol = symbols.clone().choose(&mut rng).unwrap();
                    //     let colored_string = &make_colored_string(symbol, rgb);
                    //     line += colored_string;
                }
                execute!(
                    stdout,
                    Print(line.clone())
                )?; 
            }
            stdout.flush()?;
            for seq in &mut sequences {
                seq.step();
            }
            sequences.retain(|x| x.get_top_coord()[0]-x.len() < rows.into());
        }
        stdout.execute(Clear(terminal::ClearType::All))?;
        stdout.execute(Show)?;
        disable_raw_mode()?;
        return Ok(());
    }
}

