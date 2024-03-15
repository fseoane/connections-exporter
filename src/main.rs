use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let tcp = TcpStream::connect("20.12.69.250:25022").unwrap();
    
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("username", "password").unwrap();
    
    let command: &str = "show version";
    
    
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command).unwrap();

    let mut return_command = String::new();
    channel.read_to_string(&mut return_command).unwrap();
    println!("{}", s);

    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}

