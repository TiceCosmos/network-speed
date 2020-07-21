use log::{info, warn};
use network_speed::*;
use std::io;
use std::net::{SocketAddr, TcpListener};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Protocol
    #[structopt(short = "c", long = "protocol", default_value = "tcp")]
    protocol: Protocol,
    /// Port
    #[structopt(short = "p", long = "port", default_value = "8000")]
    port: u16,
}

fn main() -> io::Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    info!("{:#?}", opt);

    let addr = SocketAddr::from(([0, 0, 0, 0], opt.port));

    let mut buffer = [0_u8; 65536];

    match opt.protocol {
        Protocol::TCP => {
            let listener = TcpListener::bind(addr)?;
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    info!("link from {}", addr);
                    loop {
                        tcp_recv_data(&mut stream, &mut buffer)?;
                        tcp_send_data(&mut stream, TRAN_SIZE)?;
                    }
                }
                Err(e) => warn!("connect: {}", e),
            }
        }
        protocol => warn!("未实现: {}", protocol),
    }

    Ok(())
}
