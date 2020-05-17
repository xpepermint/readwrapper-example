use std::pin::Pin;
use std::marker::Unpin;
use async_std::io::Read;
use async_std::task::{Context, Poll};
use async_std::io;
use async_std::net::{TcpStream};
use async_std::prelude::*;

#[async_std::main]
async fn main() {
    let mut res = request().await;
    let mut data = vec![0u8; 1024];
    res.read(&mut data).await.unwrap();

    println!("{:?}", data);
}

async fn request() -> Output<TcpStream> {
    let mut stream = TcpStream::connect("google.com:80").await.unwrap();
    stream.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();
    Output::new(stream)
}

pub struct Output<R> {
    stream: R,
}

impl<R> Unpin for Output<R> {}

impl<R> Output<R> {
    pub fn new(stream: R) -> Self {
        Self {
            stream,
        }
    }
}

impl<R: Read + Unpin> Read for Output<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}

impl<'a, R> Read for &'a Output<R>
where
    &'a R: Read,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &self.stream).poll_read(cx, buf)
    }
}
