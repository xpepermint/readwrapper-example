use std::pin::Pin;
use async_std::io::Read;
use async_std::task::{Context, Poll};
use async_std::io;
use async_std::net::{TcpStream};
use async_std::prelude::*;

#[async_std::main]
async fn main() {
    let mut stream = TcpStream::connect("google.com:80").await.unwrap();
    stream.write_all(b"GET / HTTP/1.1\r\n").await.unwrap();

    let mut res = Output::new(stream);
    let mut data = Vec::new();
    res.read_to_end(&mut data).await;

    println!("{:?}", data);
}

pub struct Output {
    stream: Pin<Box<dyn Read>>,
}

impl Output {
    pub fn new<S: 'static>(stream: S) -> Self
        where
        S: Read + Unpin,
    {
        Self {
            stream: Box::pin(stream),
        }
    }
}

impl Read for Output {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }
}

impl Read for &Output {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let s = Pin::into_inner(self);
        (&mut (&*s.stream)).read(buf);
        Pin::new(&mut (&*s.stream)).poll_read(cx, buf)
    }
}
