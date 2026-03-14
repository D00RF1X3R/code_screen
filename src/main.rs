use std::env;

use crate::code_screen::start_code;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut colors: Vec<[i32;3]> = vec![];
    
    if args.len() == 1 {
        let _ = start_code(None);
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
                /*val if val == "--Color" => {

                }*/
                _ => {}
            }
        }

        if colors.len() != 0 {
            let _ = start_code(Some(colors));
        }
    }
    
}


mod code_screen {
    use std::io::Write;
    use std::io::{self};
    use std::thread;
    use std::time::Duration;
    use crossterm::cursor::{Hide, MoveTo, Show};
    use crossterm::{ExecutableCommand, execute};
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
            colored_string = format!("\x1b[48;2;1;1;1m\x1b[38;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
        } else {
            colored_string = format!("\x1b[38;2;255;255;255m\x1b[48;2;{};{};{}m{}", rgb[0], rgb[1], rgb[2], symbol);
        }

        return colored_string;
    }

    trait Sequence {
        // Шаг очереди, обычно, изменение координаты
        fn step(&mut self);

        // Получение координаты самой первой части очереди
        fn get_top_coord(&self) -> [i32;2];

        fn len(&self) -> i32;
        fn get_symbol_by_index(&self, i: usize) -> String;
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
        fn get_top_coord(&self) -> [i32; 2] {
            return self.coords;
        }
        fn len(&self) -> i32 {
            return self.length;
        }
        fn get_symbol_by_index(&self, i: usize) -> String {
            match self.line.get(i) {
                Some (char ) => {return make_colored_string(*char, self.color)}
                None => {panic!()}
            }
        }
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
            return GlitchSequence {coords: [0, col_start], colors: rgb, length, line, mutable_indexes};
        }
    }

    impl Sequence for GlitchSequence {
        fn step(&mut self) {
            let mut rng = rand::rng();
            self.coords[0] += rng.random_range(1..3); // Рандомная скорость, мб лучше добавить как поле
        }
        fn get_top_coord(&self) -> [i32; 2] {
            return self.coords;
        }
        fn len(&self) -> i32 {
            return self.length;
        }
        fn get_symbol_by_index(&self, i: usize) -> String {
            match self.line.get(i) {
                Some (char ) => {
                    if self.mutable_indexes.contains(&(i as i8)) {
                        let color = get_random_color(&self.colors);
                        return make_colored_string(*char, color)
                    }
                    else {return make_colored_string(*char, self.colors[0]);}

                    
                }
                None => {panic!()}
            }
        }
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

    // Проверяем, есть ли хоть одна из координат строки в текущем положении курсора
    fn check_seq_on_coords(row: i32, col: i32, sequences: &Vec<Box<dyn Sequence>>) -> Option<&Box<dyn Sequence>> {
        return sequences.iter().find(|f| (row..(row+f.len())).contains(&f.get_x()) && f.get_y() == col.into());
    }


    pub fn start_code(colors: Option<Vec<[i32; 3]>>) -> Result<(), Box<dyn std::error::Error>> {
        // Статичные данные
        let colors_vec: Vec<[i32; 3]> = colors.unwrap_or([[0, 255, 0]].to_vec());
        let sequence_types: [&str; 2] = ["CodeSequence", "GlitchSequence"];
        let symbols: &str = "10"; 

        // Меняющиеся данные
        let mut stdout: io::Stdout = io::stdout();
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let mut sequences: Vec<Box<dyn Sequence>> = Vec::new();
        
        // Шакалим терминал
        enable_raw_mode()?;
        stdout.execute(Clear(terminal::ClearType::All))?;
        stdout.execute(Hide)?;


        // Бесконечный цикл падающих строк
        loop {  
            let (cols, rows) = terminal::size()?;

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
                    Event::Resize(_, _) => {},
                }
            }

            // Создание случайных строчек кода
            for _ in  0..rng.random_range(1..cols.div_ceil(75)){ // Нужно будет добавить в аргументы число, чтобы контролироовать количество строк
                let seq: Box<dyn Sequence>;
                match *sequence_types.iter().choose(&mut rng).unwrap() {
                    "CodeSequence" => {
                        seq = Box::new(CodeSeuqence::new(
                            rng.random_range(0..cols).into(),
                            get_random_color(&colors_vec),
                            rng.random_range(1..30),
                            symbols
                        ))
                    }
                    "GlitchSequence" => {
                        seq = Box::new(GlitchSequence::new(
                            rng.random_range(0..cols).into(),
                            colors_vec.clone(),
                            rng.random_range(1..30),
                            symbols
                        ));
                    }
                    _ => {
                        seq = Box::new(CodeSeuqence::new(
                            rng.random_range(0..cols).into(),
                            get_random_color(&colors_vec),
                            rng.random_range(1..30),
                            symbols
                        ))
                    }
                }
                sequences.push(seq);
            }

            // Курсор в нулевое положение, чтобы не рисовать всё по новой
            stdout.execute(MoveTo(0, 0))?;
            
            // Основной цикл отбражения строк в терминал
            let mut line: String = "".to_string();
            for row in 0..rows {
                for col in 0..cols {
                    let row_i32: i32 = row.into();
                    match check_seq_on_coords(row_i32, col.into(), &sequences) {
                        Some(seq) => {
                            let colored_string = seq.get_symbol_by_index((seq.get_top_coord()[0] - row_i32).try_into().unwrap());
                            line += &colored_string;
                        }
                        None => {
                            line += " ";
                        }
                    }
                }
                 
            }
            execute!(
                    stdout,
                    // Добавляю еще bold на весь шрифт
                    Print("\x1b[1m".to_owned() + &line.clone() + "\x1b[0m")
                )?;
            stdout.flush()?;

            // Сборка мусора
            for seq in &mut sequences {
                seq.step();
            }
            sequences.retain(|x: &Box<dyn Sequence + 'static>| x.get_top_coord()[0]-x.len() < rows.into());

            // Время между 'кадрами'
            thread::sleep(Duration::from_millis(50));
        }

        // Возвращаем терминал в обычное состояние
        stdout.execute(Clear(terminal::ClearType::All))?;
        stdout.execute(Show)?;
        disable_raw_mode()?;

        return Ok(());
    }
}

