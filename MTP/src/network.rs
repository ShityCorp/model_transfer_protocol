use std::{net::{Ipv4Addr, SocketAddr, UdpSocket}};
use gethostname::gethostname;

/// Generate the format of a alive packet.
fn generate_hearbeat_packet(){
    let h = gethostname();
    // We need to generate the mark of a specific host so we know whos connected
    // as the heartbeat should be pretty slow we will use the same information as the "up" contact list. 
    // I am thinking we can use MAC signed with private key sending to a specific host 
    // as the key will be the way we allow communication betwee hosts anyway
    
}

fn generate_socket_bind(bind: Ipv4Addr , port:u16) -> UdpSocket{
    // Create a UDP  binding location
    match UdpSocket::bind(format!("{bind}:{port}")){
        Ok(b) => return b,
        Err(e) => panic!("Could not bind it self to port {e}")

    } 
}


fn send_udp_packet(socket:&UdpSocket, addr: SocketAddr, message:&str) -> bool{
    /* 
    Send a single UDP packet to  a specific address 
    200 = message sent succesfully
    0 = message error
    */
    match socket.send_to(message.as_bytes(), addr){
        Ok(res) => {
            return true;
        }
        Err(e) => {
            return false;
        }
    }
}


// TODO : make a conf file seting up the buffer size for the UDP packets
fn heart_beat_listener(socket:&UdpSocket) -> std::io::Result<()>{
    /*
    Bind a UDP port to listen and answer back to heart_beat This will listen on the port and look for a specific formated packet 
    If the host whom we receive the packet from has pass a arbitrary time gap we answer back with our own heart beat automatically. 
    */
    // TODO: find a proper size to recognise precise hosts to know who AND on what machine they are connected
    let mut buf = [0; 10]; // Create a buffer of 10 bytes all initialized at 0
    let self_data = &mut [0; 10]; // TODO: setup what convention we want to put in there to recognize a host.
    
    loop{
        // buf receive the data form the message as it was a mutable pointer
        let (amt, src) = match socket.recv_from(&mut buf) {  // Where did we receive the message from where src will be the sender IP+port and amt is the size of the message in byte
            // Match is use to verify if we receive a OK or an error from teh recv_from and then continue the tree
            Ok(res) => res, // If we receive a valid value we continue like normal
            Err(e) => { // if we receive something too large we restart the loop and ignore it.
                continue; // Ignore the packet. If its too large it wasn't supposed to be sent to us.
            }
        };             
        let data = &mut buf[..amt]; // we take only the data up to amt lenght from buffer. Then make buf an imutable variable
         
        // TODO: call the function to see who was alive when and last.

        // TODO : modify this so we send back our own information to the host ( if a valid node ) so he knows we are up as well if last contact longer then configuration
        match send_udp_packet(socket, src, "alive") {
            true => continue,
            false => eprint!("heartbeat answer with {src} failed to be sent. Error 0"),
        }

    }
}

fn hear_beat_sender(socket:UdpSocket) -> std::io::Result<()>{
    return std::io::Result<()>;

}