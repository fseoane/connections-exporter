use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let tcp = TcpStream::connect("20.12.69.250:25022").unwrap();
    
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("user", "password").unwrap();
    

    let local_nets_ips="20.12.69.|20.12.73.|20.12.10.|20.12.192.|20.69.0.|20.12.0.|29.4.73.";
    let local_ip="2.138.62.240";
    //let command = format!("pfctl -ss | grep ' -> ' | grep 'ESTABLISHED:ESTABLISHED|SINGLE:MULTIPLE|MULTIPLE:MULTIPLE' | grep '{}' | awk '\{print ($3$4 \" \" $6)}' | grep '^[1-9]' | sort -u --key=2,2 && pfctl -ss | grep ' <- ' | grep 'ESTABLISHED:ESTABLISHED|SINGLE:MULTIPLE|MULTIPLE:MULTIPLE' | grep '{}' | awk '\{print ($5 \"() \" $3)}' | grep -v '{}' | grep '^[1-9]' | sort -u --key=1,1",local_ip,local_ip,local_nets_ips);
    let command = String::from("pfctl -ss | grep ' -> ' | grep 'ESTABLISHED:ESTABLISHED|SINGLE:MULTIPLE|MULTIPLE:MULTIPLE'");
    //let command = String::from("pfctl -ss | grep ' -> '");
    //let command = String::from("pfctl -ss");
    
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command.as_str()).unwrap();

    let mut return_command = String::new();
    channel.read_to_string(&mut return_command).unwrap();
    println!("{}", return_command);

    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}


