use std::net::TcpListener;

const BARBAROSSA_SERVER_ADDRESS:&str = "127.0.0.1:8000";




fn main() {

       println!("barbaroosa starting:  [{}]",BARBAROSSA_SERVER_ADDRESS); 
       let listener =  TcpListener::bind(BARBAROSSA_SERVER_ADDRESS).unwrap();
       println!("barbaroosa listining: [{}]",BARBAROSSA_SERVER_ADDRESS); 
       
       for stream in listener.incoming(){
            let _stream = stream.unwrap();
            println!("connection established!");
       }


}
