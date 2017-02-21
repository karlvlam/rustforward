use std::env;
use std::fs::File;
use std::io::*;
use std::net::*;
use std::{thread, time};
use std::collections::HashMap;


fn main(){

    let mut config = HashMap::new();
    match env::args().count() {
        0 | 1 => {
            show_help();
            std::process::exit(1);
        }
        _ => {
            config = read_file( env::args().nth(1).unwrap(), config);
        }
    }

    for (key, value) in config {
        thread::spawn( move|| {
            start_listener(&key, &value);
        });
    }

    loop {
        thread::sleep(time::Duration::from_millis(3600000));
    }

}

fn show_help(){
    println!("Usage: rustforward portforwardlist.conf");
}

fn read_file(path: String, mut config: HashMap<String, String>) -> HashMap<String, String> {

    let mut file = match File::open(path) {
        Err(e) => panic!("couldn't open {}", e),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(e) => panic!("couldn't read {}", e),
        Ok(_) => println!("Read config file OK!"),
    }

    let mut lines = s.lines();
    while let Some(l) = lines.next() {
        let l = l.to_string();
        let mut t = l.split_whitespace();
        let count = l.split_whitespace().count();
        match count {
            2 => {
                config.insert(t.next().unwrap().to_string(), t.next().unwrap().to_string());
            }
            _ => {}
        }
    }
    config
}


fn start_listener(src_addr: &String, dest_addr: &String) {
    let listener = TcpListener::bind(src_addr.as_str()).unwrap();
    println!("Port forward started {} -> {}", src_addr, dest_addr);
    for stream in listener.incoming(){
        let dest_addr = dest_addr.clone();
        match stream{
            Ok(stream) => {
                thread::spawn( move|| {
                    handle_client(stream, &dest_addr);
                });
            }
            Err(_) => {
                println!("sth error!");
            }

        }
    }
}

fn handle_client(mut src_stream: TcpStream, dest_addr: &str){

    let dest_connection = TcpStream::connect(dest_addr);
    let mut dest_stream: TcpStream;

    match dest_connection{
        Ok(stream) => {
            dest_stream = stream;
        }
        Err(_) => {
            println!("Dest Error!");
            return;
        }
    }

    let _ = src_stream.set_nonblocking(true);
    let _ = dest_stream.set_nonblocking(true);

    let mut src_buf: [u8; 128] = [0; 128];
    let mut dest_buf: [u8; 128] = [0; 128];
    loop {
        let res = src_stream.read(&mut src_buf);
        match res {
            Ok(byte_count) => {
                if byte_count == 0 {
                    let _ = src_stream.shutdown(Shutdown::Both);
                    break;
                }
                let _ = dest_stream.write(&src_buf[0..byte_count]);
                //println!("{:?}", &buf[0 .. byte_count]);
            }
            Err(e) => {
                //println!("Error: {:?}", e);
                //stream.shutdown(Shutdown::Both);
                //break;
            }
        }

        let res = dest_stream.read(&mut dest_buf);
        match res {
            Ok(byte_count) => {
                if byte_count == 0 {
                    let _ = dest_stream.shutdown(Shutdown::Both);
                    break;
                }
                let _ = src_stream.write(&dest_buf[0..byte_count]);
                //println!("{:?}", &buf[0 .. byte_count]);
            }
            Err(e) => {
                //println!("Error: {:?}", e);
                //stream.shutdown(Shutdown::Both);
                //break;
                thread::sleep(time::Duration::from_millis(5));
            }
        }

    }


}
