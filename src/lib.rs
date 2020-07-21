use log::{info, warn};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::convert::TryInto;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Instant;
use std::{fmt, io, mem, str};

pub static TRAN_SIZE: usize = 65536 * 32;

#[derive(Debug)]
pub enum Error {
    PROTOCOL(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PROTOCOL(proto) => write!(f, "unkown protocol: {:?}", proto),
        }
    }
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
}
impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for Protocol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "tcp" => Ok(Self::TCP),
            "ucp" => Ok(Self::UDP),
            _ => Err(Error::PROTOCOL(s)),
        }
    }
}

pub fn tcp_send_data(stream: &mut TcpStream, length: usize) -> io::Result<()> {
    let len = length / mem::size_of::<u64>();
    let mut data = vec![0; len];
    data[0] = u64::MAX;
    let mut small_rng = SmallRng::from_entropy();
    small_rng.fill(&mut data[1..(len - 1)]);
    data[len - 1] = data[1..(len - 1)].iter().fold(0, |a, b| a ^ b);
    for item in data.into_iter().map(|x| x.to_be_bytes()) {
        stream.write_all(&item)?;
    }
    Ok(())
}

pub fn tcp_recv_data(stream: &mut TcpStream, buffer: &mut [u8]) -> io::Result<()> {
    let mut length = 0;
    let mut check = 0;
    let mut count = 0;
    let mut stat = false;
    let mut time = Instant::now();
    loop {
        let recv_len = stream.read(&mut buffer[length..])?;
        length += recv_len;
        let mut s = 0;
        if !stat {
            while s + mem::size_of::<u64>() <= length {
                let value =
                    u64::from_be_bytes(buffer[s..(s + mem::size_of::<u64>())].try_into().unwrap());
                s += mem::size_of::<u64>();
                if value == u64::MAX {
                    stat = true;
                    time = Instant::now();
                    count = 2 * mem::size_of::<u64>();
                    break;
                }
            }
        }
        while s + mem::size_of::<u64>() <= length {
            let value =
                u64::from_be_bytes(buffer[s..(s + mem::size_of::<u64>())].try_into().unwrap());
            s += mem::size_of::<u64>();
            if count == TRAN_SIZE {
                info!(
                    "传输速度：{:>12}",
                    format!(
                        "{:.3} MB/s",
                        TRAN_SIZE as f64 * 1000000.
                            / (time.elapsed().as_micros() as f64 * 1048576.)
                    )
                );
                if check != value {
                    warn!("校验失败");
                }
                return Ok(());
            } else {
                check ^= value;
                count += mem::size_of::<u64>();
            }
        }
        if s == 0 {
            continue;
        }
        for (i, j) in (s..length).enumerate() {
            buffer[i] = buffer[j];
        }
        length -= s;
    }
}
