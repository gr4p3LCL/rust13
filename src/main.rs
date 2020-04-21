use std::env;

fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => (c as u8 + 13) as char,
            'N'..='Z' | 'n'..='z' => (c as u8 - 13) as char,
            _ => c,
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = if let Some(message) = args.get(1) {
        message
    } else {
        "Cebtenzzvat Cenkvf vf sha!"
    };

    println!("{}", rot13(message));
}
