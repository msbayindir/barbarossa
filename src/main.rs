use std::net::TcpListener;

const BARBAROSSA_SERVER_ADDRESS:&str = "127.0.0.1:8000";




fn main() {

       println!("barbaroosa starting: [{}]",BARBAROSSA_SERVER_ADDRESS); 
       let listener =  TcpListener::bind(BARBAROSSA_SERVER_ADDRESS);


}
