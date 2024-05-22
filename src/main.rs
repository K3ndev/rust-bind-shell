use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::env;


fn handle_connection(mut stream: TcpStream) {


    let os_info = env::consts::OS;

    let message: String;
    
    if os_info == "linux" {
        let linux_command = Command::new("whoami")
        .output()
        .expect("Failed to execute command");
        let linux_command_output = String::from_utf8_lossy(&linux_command.stdout);
        message = format!("Output:\n{}", linux_command_output);
    } else {
        message = "none".to_owned();
    }


    match stream.write_all(message.as_bytes()) {
        Ok(_) => {
            println!("Message sent successfully");
        },
        Err(e) => {
            println!("Failed to send message: {}", e);
            return;
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
                println!("Connection established");
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}



// docs
// https://doc.rust-lang.org/std/net/struct.TcpListener.html
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/process/struct.Command.html
// https://doc.rust-lang.org/std/env/consts/constant.OS.html
// https://crates.io/categories/os::windows-apis