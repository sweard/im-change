use std::process::Command;

use rdev::Key;

fn is_target_input_method(target_im: &str) -> Option<bool> {
    let output = Command::new("defaults")
        .arg("read")
        .arg("com.apple.HIToolbox")
        .arg("AppleSelectedInputSources")
        .output()
        .expect("Failed to execute command");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("✅ 当前输入法信息：\n{}", stdout);
        if stdout.contains(target_im) {
            println!("当前是目标输入法");
            Some(true)
        } else {
            println!("当前不是目标输入法");
            Some(false)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ 命令执行失败：{}", stderr);
        None
    }
}

pub fn switch_to_target_input(target_im: &str) {
    let is_target = is_target_input_method(target_im);
    if is_target == Some(true) {
        println!("Already in {} input method.", target_im);
    } else {
        println!("Switching to input method:{} ...", target_im);
        let applescript = r#"
        tell application "System Events"
            tell process "SystemUIServer"
                key code 49 using {option down, control down}
            end tell
        end tell
    "#;
        loop {
            let output = Command::new("osascript")
                .arg("-e")
                .arg(applescript)
                .output()
                .expect("failed to execute AppleScript");
            let cmd_success = output.status.success();

            if cmd_success {
                // 延迟50毫秒
                std::thread::sleep(std::time::Duration::from_millis(50));
                let is_target = is_target_input_method(target_im);
                if is_target == Some(true) {
                    println!("✅ Switched to {} via AppleScript.", target_im);
                    break;
                }
            } else {
                eprintln!(
                    "❌ AppleScript failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                break;
            }
        }
    }
}

pub fn ignore_input(key: Key) -> bool {
    // 在macOS上，返回true表示忽略输入
    if key == Key::Function || key == Key::Unknown(179) {
        true
    } else {
        false
    }
}
