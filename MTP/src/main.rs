use std::net::{IpAddr, Ipv4Addr, UdpSocket};

// TODO: scan all file present to share
fn file_scan(){}

// TODO put that on its own thread using tokyo(?)
fn heart_beat_listener(bind: IpAddr , port:u16) -> std::io::Result<()>{
    /*
    UDP bind listening for other hosts heart beat.
    */
    let socket = UdpSocket::bind(format!("{bind}:{port}"))?; // Create the binding location ( where we listen )
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
        match socket.send_to(self_data, &src){
            Ok(res) => res,
            Err(e) => {
                eprint!("Error sending hearbeat back {e}");
                continue;
            }
        }; // TODO : Probably update to be used inside of the same function as when we come online.
    }

}

fn main(){      
    let _ = heart_beat_listener(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3400);
       
}