use rdev::Key::*;
use rdev::{Event, EventType, Key, listen};
use std::env::consts::OS;
mod macos;

const TARGET_IM: &str = "com.apple.inputmethod.SCIM.ITABC";

pub fn key_to_char(key: Key, shift: bool) -> Option<char> {
    let ch = match key {
        // Letters
        KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyE => Some(if shift { 'E' } else { 'e' }),
        KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyH => Some(if shift { 'H' } else { 'h' }),
        KeyI => Some(if shift { 'I' } else { 'i' }),
        KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyL => Some(if shift { 'L' } else { 'l' }),
        KeyM => Some(if shift { 'M' } else { 'm' }),
        KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyO => Some(if shift { 'O' } else { 'o' }),
        KeyP => Some(if shift { 'P' } else { 'p' }),
        KeyQ => Some(if shift { 'Q' } else { 'q' }),
        KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyS => Some(if shift { 'S' } else { 's' }),
        KeyT => Some(if shift { 'T' } else { 't' }),
        KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyY => Some(if shift { 'Y' } else { 'y' }),
        KeyZ => Some(if shift { 'Z' } else { 'z' }),

        // Numbers and symbols
        Num0 => Some(if shift { ')' } else { '0' }),
        Num1 => Some(if shift { '!' } else { '1' }),
        Num2 => Some(if shift { '@' } else { '2' }),
        Num3 => Some(if shift { '#' } else { '3' }),
        Num4 => Some(if shift { '$' } else { '4' }),
        Num5 => Some(if shift { '%' } else { '5' }),
        Num6 => Some(if shift { '^' } else { '6' }),
        Num7 => Some(if shift { '&' } else { '7' }),
        Num8 => Some(if shift { '*' } else { '8' }),
        rdev::Key::Num9 => Some(if shift { '(' } else { '9' }),

        // Symbols
        Space => Some(' '),
        Return => Some('\n'),
        Tab => Some('\t'),
        Minus => Some(if shift { '_' } else { '-' }),
        Equal => Some(if shift { '+' } else { '=' }),
        LeftBracket => Some(if shift { '{' } else { '[' }),
        RightBracket => Some(if shift { '}' } else { ']' }),
        BackSlash => Some(if shift { '|' } else { '\\' }),
        SemiColon => Some(if shift { ':' } else { ';' }),
        Quote => Some(if shift { '"' } else { '\'' }),
        Comma => Some(if shift { '<' } else { ',' }),
        Dot => Some(if shift { '>' } else { '.' }),
        Slash => Some(if shift { '?' } else { '/' }),
        BackQuote => Some(if shift { '~' } else { '`' }),

        _ => None, // 未知键不处理
    };

    ch
}

fn main() {
    println!("⌨️ 开始监听键盘事件...");
    //
    let mut input_str: String = String::new();
    let targets = ["// ", "/* ", "/** "];
    let mut shift = false;
    let buffer_size = 10;
    
    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("键被按下： {:?}", key);
                let mac_ignore = macos::ignore_input(key);
                if mac_ignore {
                    return;
                }
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    println!("Shift 键被按下");
                    shift = true;
                }
                if let Some(c) = key_to_char(key, shift) {
                    input_str.push(c);
                } else if key == Key::Backspace {
                    input_str.pop();
                } else if input_str.len() > buffer_size {
                    input_str = input_str[input_str.len() - buffer_size..].to_string();
                } else {
                    println!("未处理按键： {:?}", key);
                }
                println!("Input string: {}", input_str);
                if targets.iter().any(|t| input_str.ends_with(t)) {
                    println!("Detected {}. Switching input method...", input_str);
                    match OS {
                        "macos" => {
                            println!("⌨️ 在macOS系统上开始处理输入法切换...");
                            macos::switch_to_target_input(TARGET_IM);
                        }
                        "windows" => {
                            println!("⌨️ 在Windows系统上开始处理输入法切换...");
                            // Windows特定逻辑将来可以实现
                            println!("Windows支持尚未实现");
                            return;
                        }
                        "linux" => {
                            println!("⌨️ 在Linux系统上开始处理输入法切换...");
                            // Linux特定逻辑将来可以实现
                            println!("Linux支持尚未实现");
                            return;
                        }
                        _ => {
                            println!("❌ 不支持的操作系统: {}", OS);
                            return;
                        }
                    }
                }
            }
            EventType::KeyRelease(key) => {
                // println!("键被释放： {:?}", key);
                // 处理按键释放事件
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    println!("Shift 键被释放");
                    shift = false;
                }
            }
            _ => {
                // 忽略其他事件，如 KeyRelease、MouseMove 等
                return;
            }
        }
    };
    if let Err(error) = listen(callback) {
        eprintln!("监听失败: {:?}", error);
    }
}
