use std::{net::{Ipv4Addr, SocketAddr, UdpSocket,hostname}};


fn generate_hearbeat_packet(){
    let h = hostname();
}

fn generate_socket_bind(bind: Ipv4Addr , port:u16) -> UdpSocket{
    // Create a UDP  binding location
    match UdpSocket::bind(format!("{bind}:{port}")){
        Ok(b) => return b,
        Err(e) => panic!("Could not bind it self to port {e}")

    } 
}


fn send_udp_packet(socket:UdpSocket, addr: SocketAddr, message:&str) -> u8{
    /* 
    Send a single UDP packet to  a specific address 
    200 = message sent succesfully
    0 = message error
    */
    match socket.send_to(as_bytes(message), addr){
        Ok(res) => {
            return 200;
        }
        Err(e) => {
            return 0;
        }
    }
}


// TODO : make a conf file seting up the buffer size for the UDP packets
fn heart_beat_listener(socket:UdpSocket) -> std::io::Result<()>{
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
            200 => continue,
            0 => eprint!("heartbeat answer with {src} failed to be sent. Error 0")

        }

    }
}

