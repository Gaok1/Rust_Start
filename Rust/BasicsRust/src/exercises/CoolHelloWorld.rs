use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

static BL: &str = "\x1B[1A"; // backline
static CL: &str = "\x1B[2K"; // clearline
static HC: &str = "\x1B[?25l"; // hide cursor
static DELAY: u64 = 20;

fn main() {
    let hello = "hello world!";

    print!("{}", HC);
    let mut string = String::from("");
    let mut characterbuffer: char;
    let loading: Vec<&str> = vec!["|", "/", "-", "\\"];
    let mut load_ite: usize = 0;

    for origin in hello.chars() {
        for i in 11..220 {
            characterbuffer = i as u8 as char;
            // sleep(Duration::from_millis(100));
            print!(
                "Loading: [{}]\n{}{} {BL}\r",
                loading.get(load_ite).expect("nao"),
                string.trim_end(),
                characterbuffer,
            );
            stdout().flush().unwrap(); // Chame stdout().flush() e adicione .unwrap() para tratar erros
            load_ite = (load_ite + 1) % 4;
            // flush da stdout
            if origin == characterbuffer {
                string.push(characterbuffer);
                break;
            }
            sleep(Duration::from_millis(DELAY)); // Chame sleep(Duration::from_millis(2)) e adicione .unwrap() para tratar erros
        }
    }
    print!(
        "{CL}Decoded!!\n{}",
        string.trim_end(),
    );
}
