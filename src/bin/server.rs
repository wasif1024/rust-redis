//#![allow(dead_code)]
//#![allow(unused_variables)]
use bytes::BytesMut;
use rust_redis::{helper::buffer_to_array, Command as cmd, Db};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let mut database_obj = Db::default();
    loop {
        let mut buffer = BytesMut::with_capacity(1024);
        let (mut socket, _addr) = listener.accept().await?;
        socket.read_buf(&mut buffer).await?;
        let attrs = buffer_to_array(&mut buffer);
        let command = cmd::get_command(&attrs[0]);
        process_query(command, attrs, socket, &mut database_obj).await?;
        //println!("connection accepted {:?}", attrs[0]);
    }
    //println!("server");
    //Ok(())
}
async fn process_query(
    command: cmd,
    attrs: Vec<String>,
    mut socket: TcpStream,
    db: &mut Db,
) -> std::io::Result<()> {
    match command {
        cmd::Get => {
            let result = db.read(&attrs);
        match result {
            Ok(result) => {
                socket.write_all(result).await?;
            }
            Err(_err) => {
                println!("no key found {:?}", _err);
                socket.write_all(b"").await?;
            }
        }
        Ok(())
        }
        cmd::Set => {
            let resp = db.write(&attrs);
            match resp{
                Ok(result)=>{
                    println!("set result: {}", result);
                    socket.write_all(result.as_bytes()).await?;
                },Err(_err)=>{
                    socket.write_all(b"").await?;
                }
            }
            Ok(()) 
        }
        cmd::Invalid => {
            Ok(())
        }
    }

}
