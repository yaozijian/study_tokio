use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
	let addr = "127.0.0.1:6142".parse().unwrap();
	/*
		异步操作如下
		1 connect: 返回 ConnectFuture，是一个 Future
		2 write_all: 返回 WriteAll<TcpStream,&str>，是一个 Future

		Future 是实现了 tokio::future::Future 特性的对象

		Future 特性提供了众多方法，包括 and_then,then,map_err
		1 and_then  当前Future成功完成之后,执行另一个Future。注意：
			(1) 当前Future的所有权被转移
			(2) 闭包的参数是当前Future成功的结果(std::result::Result::Ok)
		2 then  当前Future完成之后,执行另一个Future。注意:
			(1) 当前Future的所有权被转移
			(2) 闭包的参数是当前Future的结果(std::result::Result)
		3 map_err   如果当前Future失败，则执行另一个Future，处理错误，返回不同的错误类型。注意:
			(1) 当前Future的所有权被转移
			(2) 闭包的参数是当前Future失败的结果(std::result::Result::Err)
	*/
	let client = TcpStream::connect(&addr)
		.and_then(|stream| {
			println!("连接成功");
			io::write_all(stream, "Hello world\n")
				.then(|result| {
					println!("写入成功: {}", result.is_ok());
					Ok(())
				})
		})
		.map_err(|err| {
			println!("连接失败: {:?}", err);
		});

	println!("即将开始运行");
	tokio::run(client);
	println!("运行完成");
}
