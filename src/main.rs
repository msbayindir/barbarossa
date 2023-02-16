use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};

const BARBAROSSA_SERVER_ADDRESS:&str = "127.0.0.1:8000";




fn main() {

       println!("barbaroosa starting:  [{}]",BARBAROSSA_SERVER_ADDRESS); 
       let listener =  TcpListener::bind(BARBAROSSA_SERVER_ADDRESS).unwrap();
       println!("barbaroosa listining: [{}]",BARBAROSSA_SERVER_ADDRESS); 
       
       for stream in listener.incoming(){
            let _stream = stream.unwrap();
            
            handele_connection(_stream);
       }


}

fn handele_connection(mut stream:TcpStream){
     
     let mut buffer = [0;1024];
     let len = stream.read(&mut buffer).unwrap();
     let message = String::from_utf8_lossy(&buffer[..len]);
     println!("recived : {}",message);


     let _ = stream.write_all(&buffer[..len]);
     println!("sent : {} ",message);

}
