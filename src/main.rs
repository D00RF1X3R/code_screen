use std::env;

use crate::code_screen::start_code;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut colors: Vec<[i32;3]> = vec![];
    let mut amount: i16 = 50;

    if args.len() == 1 {
        let _ = start_code(None, amount.try_into().expect("Ебло утиное"));
    } else {
        for arg in 1..args.len() {
            let argument = &args[arg];
            match argument {
                // Аргумент цвета -c --color, принимается в виде r,g,b,r,g,b,...,r,g,b
                val if val == "-c" || val == "--color" => {
                    let colors_arg = &args[arg+1];

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
                }
                val if val == "-a" || val == "--Amount" => {
                    amount = 101 - args[arg+1].parse::<i16>().expect("Насрано"); 
                }
                /*val if val == "--Color" => {

                }*/
                _ => {}
            }
        }

        if colors.len() != 0 {
            let _ = start_code(Some(colors), amount.try_into().expect("Ебло утиное"));
        }
    }
    
}


mod code_screen {
    use std::io::Write;
    use std::io::{self};
    use std::thread;
    use std::time::{Duration, Instant};
    use crossterm::cursor::{Hide, MoveTo, Show};
    use crossterm::{ExecutableCommand};
    use crossterm::event::{Event, KeyCode, poll, read};
    use crossterm::style::{Print,};
    use crossterm::terminal::{self, Clear, disable_raw_mode, enable_raw_mode};
    use rand::{RngExt};
    use rand::seq::{IndexedRandom, IteratorRandom};

    // Красим символы с помощью ANSI escape code
    fn make_colored_string(symbol: char, rgb: [i32; 3]) -> String {
        // "\x1b[38;2;255;255;255m"
        let colored_string: String;
        if symbol != ' '{
            //colored_string = format!("\x1b[48;2;1;1;1m\x1b[38;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
            colored_string = format!("\x1b[38;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
        } else {
            //colored_string = format!("\x1b[38;2;255;255;255m\x1b[48;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
            colored_string = format!("\x1b[48;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
        }

        return colored_string;
    }

    trait Sequence {
        // Шаг очереди, обычно, изменение координаты
        fn step(&mut self);
        fn draw(&self, stdout: &mut io::Stdout);

        // Получение координаты самой первой части очереди
        fn get_top_coord(&self) -> [i32;2];

        fn len(&self) -> i32;
        //fn get_symbol_by_index(&self, i: usize) -> String;
        fn get_x(&self) -> i32;
        fn get_y(&self) -> i32;
    }

    // Обычная очередь
    struct CodeSeuqence { //TODO: Сделать несколько видов секвенсов
        coords: [i32; 2],
        color: [i32; 3],
        line: Vec<char>,
        length: i32,
    }

    // Очередь с изменяющимися цветами на случайном наборе символов
    struct GlitchSequence {
        coords: [i32; 2],
        colors: Vec<[i32; 3]>,
        line: Vec<char>,
        length: i32,
        last_step: i8,
        mutable_indexes: Vec<i8> // Надо сделать так, чтобы цвет конкретного символа менялся
    }

    impl CodeSeuqence {
        pub fn new(col_start:i32, rgb: [i32; 3], length: i32, chars: &str) -> Self {
            let chars_iter = chars.chars();

            let mut line: Vec<char> = vec![];
            let mut rng = rand::rng();

            for _ in 0..length {
                line.push(chars_iter.clone().choose(&mut rng).unwrap());
            }
            return CodeSeuqence {coords: [0, col_start], color: rgb, length, line};
        }
    }

    impl Sequence for CodeSeuqence {
        fn step(&mut self) {
            self.coords[0] += 1
        }
        fn draw(&self, stdout: &mut io::Stdout) {
            
            for i in 0..self.length {
                if self.coords[0] - i + 1 > 0 {
                    let coords: [u16;2] = [(self.coords[0] - i).try_into().unwrap(), self.coords[1].try_into().unwrap()];
                    let _ = stdout.execute(MoveTo(coords[1], coords[0]));
                    let symbol;
                    match self.line.get(i as usize) {
                        Some(char) => {symbol = make_colored_string(*char, self.color)}
                        None => {panic!()}
                    }
                    let _ = stdout.execute(Print(symbol));
                }
            }
            // Так как рисуем с хвоста, нужно делать поправку
            if self.coords[0] - self.length + 1 > 0 {
                let coords: [u16;2] = [(self.coords[0] - self.length).try_into().unwrap(), self.coords[1].try_into().unwrap()];
                let _ = stdout.execute(MoveTo(coords[1], coords[0]));
                let _ = stdout.execute(Print(' '));
            }

        }
        fn get_top_coord(&self) -> [i32; 2] {
            return self.coords;
        }
        fn len(&self) -> i32 {
            return self.length;
        }
        // fn get_symbol_by_index(&self, i: usize) -> String {
        //     match self.line.get(i) {
        //         Some (char ) => {return make_colored_string(*char, self.color)}
        //         None => {panic!()}
        //     }
        // }
        fn get_x(&self) -> i32 {
            self.coords[0]
        }
        fn get_y(&self) -> i32 {
            self.coords[1]
        }
        
    }

    impl GlitchSequence {
        pub fn new(col_start:i32, rgb: Vec<[i32; 3]>, length: i32, chars: &str) -> Self {
            let chars_iter = chars.chars();

            let mut line: Vec<char> = Vec::new();
            let mut rng = rand::rng();
            let mut mutable_indexes = Vec::new();

            for i in 0..length {
                line.push(chars_iter.clone().choose(&mut rng).unwrap());
                // 33% шанс, что символ будет менять свой цвет
                if rng.random_range(0..3) == 0 {
                    mutable_indexes.push(i.try_into().unwrap());
                }
            }
            return GlitchSequence {coords: [0, col_start], colors: rgb, length, line, mutable_indexes, last_step: 1};
        }
    }

    impl Sequence for GlitchSequence {
        fn step(&mut self) {
            let mut rng = rand::rng();
            self.coords[0] += rng.random_range(1..3); // Рандомная скорость, мб лучше добавить как поле
        }
        fn draw(&self, stdout: &mut io::Stdout) {
            
            for i in 0..self.length {
                if self.coords[0] - i + 1 > 0 {
                    
                    let coords: [u16;2] = [(self.coords[0] - i).try_into().unwrap(), self.coords[1].try_into().unwrap()];
                    let _ = stdout.execute(MoveTo(coords[1], coords[0]));
                    let symbol;
                    match self.line.get(i as usize) {
                        Some(char) => {
                                if self.mutable_indexes.contains(&i.try_into().unwrap()) {
                                    symbol = make_colored_string(*char, get_random_color(&self.colors))
                                }
                                else {symbol = make_colored_string(*char, self.colors[0])}
                            }
                        None => {panic!()}
                    }
                    let _ = stdout.execute(Print(symbol));
                }
            }
            // Так как рисуем с хвоста, нужно делать поправку
            for i in 0..self.last_step as i32 + 1 {
                if self.coords[0] - self.length + i > 0 {
                    let coords: [u16;2] = [(self.coords[0] - self.length + i - 1).try_into().unwrap(), self.coords[1].try_into().unwrap()];
                    let _ = stdout.execute(MoveTo(coords[1], coords[0]));
                    let _ = stdout.execute(Print(' '));
                }
            }
            
        }
        fn get_top_coord(&self) -> [i32; 2] {
            return self.coords;
        }
        fn len(&self) -> i32 {
            return self.length;
        }
        // fn get_symbol_by_index(&self, i: usize) -> String {
        //     match self.line.get(i) {
        //         Some (char ) => {
        //             if self.mutable_indexes.contains(&(i as i8)) {
        //                 let color = get_random_color(&self.colors);
        //                 return make_colored_string(*char, color)
        //             }
        //             else {return make_colored_string(*char, self.colors[0]);}

                    
        //         }
        //         None => {panic!()}
        //     }
        // }
        fn get_x(&self) -> i32 {
            self.coords[0]
        }
        fn get_y(&self) -> i32 {
            self.coords[1]
        }

    }

    // Получаем случайный цвет из вектора цветов
    pub fn get_random_color(colors_vec: &Vec<[i32; 3]>) -> [ i32; 3 ] {
        let mut rng = rand::rng();
        colors_vec.clone().choose(&mut rng).unwrap().clone()
    }
    fn check_if_available(sequences: &Vec<Box<dyn Sequence>>, col_start: i32) -> bool {
        let mut flag = true;
        for i in 0..sequences.len() {
            let seq_col = sequences[i].get_x();
            let seq_y = sequences[i].get_y();
            let seq_lentgth = sequences[i].len();
            if seq_col == col_start {
                if (seq_y..(seq_y-seq_lentgth)).contains(&0) {
                    flag = false;
                }
            }
        }
        return flag
    }

    pub fn start_code(colors: Option<Vec<[i32; 3]>>, amount: u16) -> Result<(), Box<dyn std::error::Error>> {
        // Статичные данные
        let colors_vec: Vec<[i32; 3]> = colors.unwrap_or([[0, 255, 0]].to_vec());
        let sequence_types: [&str; 2] = ["CodeSequence", "GlitchSequence"];
        let symbols: &str = "0123456789qwertyuiop[]asdfghjkl;'\\zxcvbnm,./QWERTYUIOP\\{}ASDFGHJKL:|ZXCVBNM<>?"; 

        // Меняющиеся данные
        let mut stdout: io::Stdout = io::stdout();
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut sequences: Vec<Box<dyn Sequence>> = Vec::new();
        
        // Шакалим терминал
        enable_raw_mode()?;
        stdout.execute(Clear(terminal::ClearType::All))?;
        stdout.execute(Hide)?;

        let (mut cols,mut rows) = terminal::size()?;
        //let mut line: String = "".to_string();
        // for _row in 0..rows {
        //     for _col in 0..cols {
        //         line += &make_colored_string(' ', [1, 1, 1]);
        //     }
        // }
        // execute!(
        //         stdout,
        //         // Добавляю еще bold на весь шрифт
        //         Print("\x1b[1m".to_owned() + &line.clone() + "\x1b[0m")
        //     )?;
        // stdout.flush()?;
        // stdout.execute(MoveTo(0, 0))?;

        // Бесконечный цикл падающих строк
        loop {  
            // Время начала выполнения всех циклов
            let time_start = Instant::now();
            // Обработка действий в терминале
            if poll(Duration::ZERO)? {
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
                    Event::Resize(_, _) => {(cols, rows) = terminal::size()?;},
                }
            }

            // Создание случайных строчек кода
            for _ in  0..rng.random_range(1..cols.div_ceil(amount)){ // Нужно будет добавить в аргументы число, чтобы контролироовать количество строк
                let seq: Box<dyn Sequence>;
                let col_start = rng.random_range(0..cols).into();
                if check_if_available(&sequences, col_start) {
                    match *sequence_types.iter().choose(&mut rng).unwrap() {
                        "CodeSequence" => {
                            seq = Box::new(CodeSeuqence::new(
                                rng.random_range(0..cols).into(),
                                get_random_color(&colors_vec),
                                rng.random_range(1..30),
                                symbols
                            ));
                            seq.draw(&mut stdout);
                        }
                        "GlitchSequence" => {
                            seq = Box::new(GlitchSequence::new(
                                rng.random_range(0..cols).into(),
                                colors_vec.clone(),
                                rng.random_range(1..30),
                                symbols
                            ));
                            seq.draw(&mut stdout);
                        }
                        _ => {
                            seq = Box::new(CodeSeuqence::new(
                                rng.random_range(0..cols).into(),
                                get_random_color(&colors_vec),
                                rng.random_range(1..30),
                                symbols
                            ));
                            seq.draw(&mut stdout);
                        }
                    }
                    sequences.push(seq);
                }
                
            }

            // Сборка мусора
            for seq in &mut sequences {
                seq.step();
                seq.draw(&mut stdout);
            }
            sequences.retain(|x: &Box<dyn Sequence + 'static>| x.get_top_coord()[0] - x.len() < (rows+1).into() );

            // Разница между временем начала и конца выполнения всех циклов
            let mut time_diff = Duration::ZERO;
            if time_start > Instant::now() {
                time_diff = Instant::now() - time_start
            }
            
            // Время между 'кадрами', динамически изменяемое от времени на выполнение всех циклов, чтобы поддерживать стабильные кадры
            thread::sleep(Duration::from_millis(17) - time_diff);
            
            stdout.flush()?;
        }

        // Возвращаем терминал в обычное состояние
        stdout.execute(Clear(terminal::ClearType::All))?;
        stdout.execute(Show)?;
        disable_raw_mode()?;

        return Ok(());
    }
}

