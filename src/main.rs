use oxidiser_models::{recv, send};
use std::io::BufRead;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use tracing::info;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    info!("that went ok");
    let mut stream = TcpStream::connect("127.0.0.1:3500")?;
    stream.write(
        &send::Packet::Auth {
            username: "samlol".into(),
        }
        .encoded()?,
    )?;

    let mut queue = Vec::new();

    loop {
        // stream.write(&[1])?;
        // stream.read(&mut [0; 128])?;
        let mut buf = Vec::new();
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\0', &mut buf)?;
        // info!("read {:?}", buf);

        queue.append(
            &mut buf
                .split(|b| *b == b'\0')
                .filter_map(|s| {
                    // info!("{}", s);
                    serde_json::from_slice::<recv::Packet>(s).ok()
                })
                .rev()
                .collect::<Vec<_>>(),
        );

        info!(
            "queue {:?}",
            if queue.is_empty() { continue } else { &queue }
        );

        let top = if let Some(top) = queue.pop() {
            top
        } else {
            continue;
        };

        match top {
            recv::Packet::Ping => {
                stream.write(&send::Packet::Pong.encoded()?)?;
                info!("Responded to ping");
            }
            recv::Packet::ConnectionResult { result, error } => if result == 0 {},
            recv::Packet::Disconnected { reason } => {
                info!("Disconnected because {}", reason);
            }
            other => println!("{:?}", other),
        }
    }
} // the stream is closed here
