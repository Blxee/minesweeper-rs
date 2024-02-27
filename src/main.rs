use termion::{clear, color, cursor, input::TermRead, raw, style};

fn main() {
    
    println!("{}{}{}{}{}Hello, world!",
        clear::All,
        cursor::Goto(1, 1),
        color::Fg(color::Red),
        color::Bg(color::Cyan),
        style::Bold);
    std::io::stdin().events();
}
