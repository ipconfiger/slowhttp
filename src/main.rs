use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

#[macro_use]
extern crate log;
extern crate simple_logger;
use log::{info, error, warn};
use bytes::BytesMut;
use chrono::{DateTime, Utc, Datelike, Timelike};
use clap::{App, Arg};

#[warn(dead_code)]
fn simple_logger_level(){
    simple_logger::init_with_level(log::Level::Info).unwrap();
}



fn format_current_time() -> String {
    let current_time = Utc::now();
    current_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

const VERSION: &str = "HTTP/1.0 200 OK\n";
const CHARSET: &str = "Content-Type: text/plain;charset=UTF-8\n";
const LENGTH: &str = "Content-Length: ";


#[derive(Debug, Clone)]
struct Config {
    msg: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger_level();
    let mut config = Config{ msg: "".to_string() };

    let args = App::new("Server Configuration")
        .arg(Arg::with_name("port")
            .short('p')
            .long("port")
            .help("服务器运行端口")
            .required(true)
            .takes_value(true)
            .default_value("9527"))
        .arg(Arg::with_name("message")
            .short('m')
            .long("message")
            .help("给对方的留言")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("times")
            .short('t')
            .long("times")
            .help("重复的次数")
            .required(true)
            .takes_value(true)).get_matches();
    let binding = "8080".to_string();
    let default_msg = "fuck".to_string();
    let port = args.get_one::<String>("port").unwrap_or(&binding).clone();
    let msg = args.get_one::<String>("message").unwrap_or(&default_msg);
    let times = args.get_one::<String>("times").unwrap_or(&"60".to_string()).clone();
    config.msg = msg.clone();
    let ct = times.parse::<i32>().expect("times需要数字");

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    loop {
        let cfg = config.clone();
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = BytesMut::new();
            buf.resize(1024, 0);
            if let Ok(n) = socket
                .read(&mut buf)
                .await{
            }
            match handle_response(&mut socket, &cfg.msg, ct).await {
                Ok(())=>{},
                Err(err)=>{
                    error!("执行报错:{err:?}");
                }
            }
        });
    }
}

async fn handle_response(socket: &mut TcpStream, message: &String, ct: i32) -> Result<(), Box<dyn Error>> {
    let msg_len = (message.len() as i32 + 1) * ct;
    socket.write(VERSION.as_bytes()).await?;
    socket.write(format!("Date: {}\n", format_current_time()).as_bytes()).await?;
    socket.write(CHARSET.as_bytes()).await?;
    socket.write(format!("{LENGTH} {msg_len}\n").as_bytes()).await?;
    socket.write("\n".as_bytes()).await?;
    for i in 0..ct {
        socket.write(format!("{}\n", &message).as_bytes()).await?;
        socket.flush().await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    socket.shutdown().await?;
    Ok(())
}