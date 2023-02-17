

use tokio::{net::{TcpListener, TcpStream}, io::{ AsyncReadExt, AsyncWriteExt}, time::{sleep,Duration}};

const BARBAROSSA_SERVER_ADDRESS:&str = "127.0.0.1:8000";



#[tokio::main]
async fn main() {


       println!("barbaroosa starting:  [{}]",BARBAROSSA_SERVER_ADDRESS); 
       let listener =  TcpListener::bind(BARBAROSSA_SERVER_ADDRESS).await.unwrap();
       println!("barbaroosa listining: [{}]",BARBAROSSA_SERVER_ADDRESS); 
       loop {
           let (stream,_) = listener.accept().await.unwrap();
           tokio::spawn(async move{
               handele_connection(stream).await;
           });
       }
      

}

async fn handele_connection(mut stream:TcpStream){
     
     let mut buffer = [0;1024];
     let len = stream.read(&mut buffer).await.unwrap();
     let message = String::from_utf8_lossy(&buffer[..len]);
     if message.to_string() == "sefa\n".to_string(){
          sleep(Duration::from_millis(20000)).await;
          println!("sefa recived : {}",message);
          let _ = stream.write_all(&buffer[..len]).await.unwrap();
          println!("sefa sent : {} ",message);

     } else
     {
          println!("recived : {}",message);
          let _ = stream.write_all(&buffer[..len]).await.unwrap();
          println!("sent : {} ",message);
     }

     
}
