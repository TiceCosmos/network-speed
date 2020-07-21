use log::{info, warn};
use network_speed::*;
use std::io;
use std::net::{IpAddr, SocketAddr, TcpStream};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Protocol
    #[structopt(short = "c", long = "protocol", default_value = "tcp")]
    protocol: Protocol,
    /// IP
    #[structopt(short = "i", long = "ip", default_value = "127.0.0.1")]
    ip: IpAddr,
    /// Port
    #[structopt(short = "p", long = "port", default_value = "8000")]
    port: u16,
}

fn main() -> io::Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    info!("{:#?}", opt);

    let addr = SocketAddr::from((opt.ip, opt.port));

    let mut buffer = [0_u8; 65536];

    match opt.protocol {
        Protocol::TCP => {
            let mut stream = TcpStream::connect(addr)?;
            loop {
                tcp_send_data(&mut stream, TRAN_SIZE)?;
                tcp_recv_data(&mut stream, &mut buffer)?;
            }
        }
        protocol => warn!("未实现: {}", protocol),
    }

    Ok(())
}
