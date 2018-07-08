use std::env;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let server = env::args().len() == 2 && env::args().nth(1).unwrap() == "--server";
    if server {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        for b in buf.iter_mut() {
            *b += 1;
        }
        socket.send_to(buf, &src)?;
    } else {
        let socket = UdpSocket::bind("127.0.0.1:34255")?;
        socket.connect("127.0.0.1:34254").expect("failed to connect");
        let mut buf = [1; 10];
        println!("sending {:?}", buf);
        socket.send(&buf).expect("failed to send message");
        socket.recv(&mut buf).expect("failed to recv message");
        println!("received {:?}", buf);
    }
    Ok(())
}
