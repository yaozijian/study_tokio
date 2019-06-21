use tokio;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
	let addr = "127.0.0.1:6142".parse().unwrap();
	let lis = TcpListener::bind(&addr).unwrap();
	let server = lis.incoming().for_each(|stream| {
		let a = format!("{} -> {}", stream.peer_addr().unwrap(), stream.local_addr().unwrap());
		println!("连接建立: {}", a);

		let (r, w) = stream.split();
		let forward = io::copy(r, w);
		// 这里必须使用move: 因为future不是立即执行的,如果不move,则future执行时,可能环境变量a已经失效
		let forward = forward.then(move |t| {
			match t {
				Ok((cnt, _, _)) => println!("连接 {} 复制转发了 {} 字节", a, cnt),
				Err(e) => println!("连接 {} 复制转发错误: {:?}", a, e),
			}
			Ok(())
		});
		// 注意: 必须spawn才能生效
		tokio::spawn(forward);
		Ok(())
	}).map_err(|e| println!("接收连接错误: {:?}", e));

	tokio::run(server);
}
