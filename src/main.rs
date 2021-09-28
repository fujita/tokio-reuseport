use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;

async fn serve(idx: usize) {
    let addr: std::net::SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let sock = socket2::Socket::new(
        match addr {
            SocketAddr::V4(_) => socket2::Domain::IPV4,
            SocketAddr::V6(_) => socket2::Domain::IPV6,
        },
        socket2::Type::STREAM,
        None,
    )
    .unwrap();

    sock.set_reuse_address(true).unwrap();
    sock.set_reuse_port(true).unwrap();
    sock.set_nonblocking(true).unwrap();
    sock.bind(&addr.into()).unwrap();
    sock.listen(1024).unwrap();

    let mut incomming =
        tokio_stream::wrappers::TcpListenerStream::new(TcpListener::from_std(sock.into()).unwrap());
    loop {
        let conn = incomming.next().await;
        if let Some(conn) = conn {
            match conn {
                Ok(conn) => {
                    println!("worker {}: Accepted new conn: {:?}", idx, conn);
                    tokio::spawn(async move {
                        // handle client
                    });
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut handlers = Vec::new();
    for i in 0..num_cpus::get() {
        let h = std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(serve(i));
        });
        handlers.push(h);
    }

    for h in handlers {
        h.join().unwrap();
    }
}
