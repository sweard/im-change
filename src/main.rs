use rdev::Key::*;
use rdev::{Event, EventType, Key, listen};
use std::process::Command;

const TARGET_IM: &str = "com.apple.inputmethod.SCIM.ITABC";

fn switch_to_chinese_input() {
    let applescript = r#"
        tell application "System Events"
            tell process "SystemUIServer"
                key code 49 using {option down, control down}
            end tell
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(applescript)
        .output()
        .expect("failed to execute AppleScript");

    if output.status.success() {
        println!("✅ Switched to Chinese input method via AppleScript.");
    } else {
        eprintln!(
            "❌ AppleScript failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn get_input_method() -> String {
    let output = Command::new("defaults")
        .arg("read")
        .arg("com.apple.HIToolbox")
        .arg("AppleSelectedInputSources")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("✅ 当前输入法信息：\n{}", stdout);
        if stdout.contains(TARGET_IM) {
            println!("当前是中文拼音输入法");
            TARGET_IM.to_owned()
        } else {
            println!("当前不是中文拼音输入法");
            String::new()
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ 命令执行失败：{}", stderr);
        String::new()
    }
}

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

    let mut input_str: String = String::new();
    let targets = ["// ", "/* ", "/** "];
    let mut shift = false;
    let buffer_size = 10;

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("键被按下： {:?}", key);
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    println!("Shift 键被按下");
                    shift = true;
                }
                if let Some(c) = key_to_char(key, shift) {
                    input_str.push(c);
                } else if key == Key::Backspace {
                    input_str.pop();
                } else if input_str.len() > buffer_size {
                    input_str.remove(0);
                } else {
                    println!("未处理按键： {:?}", key);
                }
                println!("Input string: {}", input_str);
                if targets.iter().any(|t| input_str.ends_with(t)) {
                    println!("Detected {}. Switching input method...", input_str);
                    let cur_method = get_input_method();
                    if cur_method == TARGET_IM {
                        println!("Already in Chinese input method.");
                    } else {
                        println!("Switching to Chinese input method...");
                        switch_to_chinese_input();
                        input_str.clear();
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
