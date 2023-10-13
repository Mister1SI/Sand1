use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write, stdin, stdout};
use std::{fs, process};

//Layout: S<file_size: u128>I<filename_size: u32>F<filename>D<data>


//Print help text
pub fn help() {
    let help_text = r#"
a
help meu
    "#;
    println!("{}", help_text);
}

pub fn client(addr: String) {
    
    //Attempt to connect to the server
    let mut stream = match TcpStream::connect(addr) {
        Ok(f) => f,
        Err(e) => {
            println!("Error connecting to server: {}", e);
            process::exit(0);
        }
    };
    
    let mut data_buf = String::new();
    let mut filename = String::new();
    let mut query_filename = true;
    while query_filename {
        let res = client_read();
        if let Ok(s) = res.0 {
            data_buf = s;
            filename = res.1;
            query_filename = false;
        }
    }
    
    
    let file_size = data_buf.len() as u128;
    let filename_size = filename.len() as u32;
    
    let mut buf: Vec<u8> = Vec::new();
    
    // Append the "S" and file_size to the buffer
    buf.write_all(b"S").expect("Error writing 'S' to buffer");
    buf.write_all(&file_size.to_be_bytes()).expect("Error writing file_size to buffer");
    
    // Append the "I" and filename_size to the buffer
    buf.write_all(b"I").expect("Error writing 'I' to buffer");
    buf.write_all(&filename_size.to_be_bytes()).expect("Error writing filename_size to buffer");
    
    // Append the "F" and the filename as bytes to the buffer
    buf.write_all(b"F").expect("Error writing 'F' to buffer");
    buf.write_all(filename.as_bytes()).expect("Error writing filename to buffer");
    
    // Append the "D" and the data_buf as bytes to the buffer
    buf.write_all(b"D").expect("Error writing 'D' to buffer");
    buf.write_all(data_buf.as_bytes()).expect("Error writing data_buf to buffer");
    
    if let Err(e) = stream.write_all(&buf) {
        println!("Error writing to the data stream: {}", e);
        process::exit(0);
    }
    
    
}

//Function for querying the user for a filename and reading its results
fn client_read() -> (Result<String, io::Error>, String) {
    print!("Enter a filename\n> ");
    if let Err(e) = stdout().flush() {
        println!("Error writing to stdout: {}", e);
        process::exit(0);
    }
    
    let mut buf = String::from("");
    if let Err(e) = stdin().read_line(&mut buf) {
        println!("\nError reading console input: {}", e);
        process::exit(0);
    }
    
    let filename = buf.trim();
    match fs::read_to_string(filename) {
        Ok(c) => {
            return (Ok(c), filename.into());
        }
        Err(e) => {
            return (Err(e), String::from(filename));
        }
    };

    
}

pub fn server(addr: String) {
    
}


