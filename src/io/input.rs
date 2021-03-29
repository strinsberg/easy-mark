use rustyline::Editor;

pub fn get_line(prompt: &str) -> String {
    readline_with_initial(prompt, ("", ""))
}

pub fn readline_with_initial(prompt: &str, initial: (&str, &str)) -> String {
    loop {
        let mut rl = Editor::<()>::new();
        let res: String = rl
            .readline_with_initial(prompt, initial)
            .unwrap()
            .trim()
            .to_string();
        if !res.is_empty() {
            return res;
        }
        println!("\n*** Input cannot be empty ***\n");
    }
}

pub fn get_u32(prompt: &str, error_msg: &str) -> u32 {
    loop {
        let num: String = get_line(prompt);
        match num.parse::<u32>() {
            Ok(x) => break x,
            _ => println!("\n*** {} ***\n", error_msg),
        }
    }
}

pub fn get_f32(prompt: &str, error_msg: &str) -> f32 {
    loop {
        let num: String = get_line(prompt);
        match num.parse::<f32>() {
            Ok(x) => break x,
            _ => println!("\n*** {} ***\n", error_msg),
        }
    }
}
