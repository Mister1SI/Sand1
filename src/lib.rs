#![allow(unused)]

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write, stdin, stdout};
use std::{fs, process};
use std::convert::TryInto;

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
        if let Ok(s) = &res.0 {
            data_buf = s.to_string();
            filename = res.1;
            query_filename = false;
        }
        if let Err(e) = res.0 {
            println!("Error reading file: {}", e);
        }
    }
    
    
    let file_size = data_buf.len() as u128;
    let filename_size = filename.len() as u32;
    
    let mut buf: Vec<u8> = Vec::new();
    
    // Append the "S" and file_size to the buffer
    //buf.write_all(b"S").expect("Error writing 'S' to buffer");
    buf.write_all(&file_size.to_be_bytes()).expect("Error writing file_size to buffer");
    
    // Append the "I" and filename_size to the buffer
    //buf.write_all(b"I").expect("Error writing 'I' to buffer");
    buf.write_all(&filename_size.to_be_bytes()).expect("Error writing filename_size to buffer");
    
    // Append the "F" and the filename as bytes to the buffer
    //buf.write_all(b"F").expect("Error writing 'F' to buffer");
    buf.write_all(filename.as_bytes()).expect("Error writing filename to buffer");
    
    // Append the "D" and the data_buf as bytes to the buffer
    //buf.write_all(b"D").expect("Error writing 'D' to buffer");
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
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            if e.kind() == io::ErrorKind::AddrInUse {
                println!("Error creating listening server: Address is already being used.");
                process::exit(0);
            } else {
                println!("Error creating listening server: {}", e);
                process::exit(0);
            }
        }
    };
    
    for stream_in in listener.incoming() {
        let mut stream = match stream_in {
            Ok(s) => s,
            Err(e) => {
                println!("Error processing incoming connection: {}", e);
                process::exit(0);
            }
        };
        
        let mut receieved = Vec::new();
        if let Err(e) = stream.read_to_end(&mut receieved) {
            println!("Error reading from client: {}", e);
            process::exit(0);
        }
        
        //Format: <filesize: u128><filename_size: u32><filename><data>
        let (filesize, filename_size, filename, data, receieved) = parse_incoming_data(receieved);
        
        println!("Filesize: {filesize}\nFilename Size: {filename_size}\nFilename: {filename}");
        
        let mut buf = String::new();
        let mut check_write = true;
        while check_write {
            println!("FILE DETAILS\nFilename: \"{filename}\"\nFile size: \"{filesize}\"\nSave file?(y/n) ");
            io::stdout().flush().unwrap();
            
            
            if let Err(e) = io::stdin().read_line(&mut buf) {
                println!("Error reading from standard input: {}", e);
                continue;
            }
            
            let cmp = &buf[0..0];
            
            if str::eq_ignore_ascii_case(cmp, "y") {
                check_write = false;
            } else if str::eq_ignore_ascii_case(cmp, "n") {
                println!("Cancelling file write");
                process::exit(0);
            }
        }
        
        let file = fs::OpenOptions::new().write(true).create_new(true).open(filename.as_str());
        
        match file {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&data[..]) {
                    println!("Error writing to file {}: {}", filename, e);
                    process::exit(0);
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    println!("Error creating file: {} already exists.", filename);
                    process::exit(0);
                } else {
                    println!("Error creating file: {}", e);
                    process::exit(0);
                }
            }
        }
        
        process::exit(0);
    }
    
    
}

fn parse_incoming_data(received: Vec<u8>) -> (u128, u32, String, Vec<u8>, Vec<u8>) {
    let mut offset = 0;

    // Parse <filesize: u128>
    let filesize_bytes: [u8; 16] = received[offset..offset + 16]
        .try_into()
        .expect("Expected 16 bytes for filesize");
    let filesize: u128 = u128::from_be_bytes(filesize_bytes);
    offset += 16;

    // Parse <filename_size: u32>
    let filename_size_bytes: [u8; 4] = received[offset..offset + 4]
        .try_into()
        .expect("Expected 4 bytes for filename size");
    let filename_size: u32 = u32::from_be_bytes(filename_size_bytes);
    offset += 4;

    // Parse <filename>
    let filename_bytes: Vec<u8> = received[offset..(offset + filename_size as usize)]
        .to_vec(); // Assuming UTF-8 filename, convert to Vec<u8> for simplicity
    let filename = String::from_utf8(filename_bytes)
        .expect("Failed to parse filename as UTF-8");
    offset += filename_size as usize;

    // The remaining data is <data>
    let data = received[offset..].to_vec();

    // Return a tuple with the parsed values
    (filesize, filename_size, filename, data, received)
}

