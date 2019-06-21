use std::io::Cursor;

use bytes::{Buf, Bytes};
use futures::{Async, Future, try_ready};
use tokio::io::AsyncWrite;
use tokio::net::tcp::{ConnectFuture, TcpStream};

struct GetAddr {
	conn: ConnectFuture,
}

impl Future for GetAddr {
	type Item = String;
	type Error = <ConnectFuture as Future>::Error;
	fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
		let sock = try_ready!(self.conn.poll());
		let addr = format!("{} -> {}", sock.local_addr().unwrap(), sock.peer_addr().unwrap());
		Ok(Async::Ready(addr))
	}
}

//----------------------

enum Hello {
	Connecting(ConnectFuture),
	Connected(TcpStream, &'static [u8], Cursor<Bytes>),
}

impl Future for Hello {
	type Item = ();
	type Error = std::io::Error;
	fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
		loop {
			match self {
				Hello::Connecting(ref mut conn) => {
					let s = try_ready!(conn.poll());
					let d = Cursor::new(Bytes::from_static(b"Hello World\n"));
					*self = Hello::Connected(s, "hello world\n".as_bytes(), d);
				},
				Hello::Connected(ref mut sock, dat, cur) => {
					while cur.has_remaining() {
						try_ready!(sock.write_buf(cur));
					}
					let mut w = tokio::io::write_all(sock, dat);
					try_ready!(w.poll());
					return Ok(Async::Ready(()));
				},
			}
		}
	}
}

fn main() {
	let addr = "127.0.0.1:6142".parse().unwrap();
	let conn = TcpStream::connect(&addr);
	let getit = GetAddr { conn }.and_then(move |str| {
		println!("连接成功: {}", str);
		let conn = TcpStream::connect(&addr);
		let hello = Hello::Connecting(conn).map_err(|e| println!("IO错误: {:?}", e));
		tokio::spawn(hello);

		Ok(())
	}).map_err(|e| println!("连接失败: {}", e));
	tokio::run(getit);
}
