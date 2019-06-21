use futures::{Async, Future, try_ready};

struct HelloWorld;

impl Future for HelloWorld {
	type Item = String;
	type Error = ();
	fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
		Ok(Async::Ready("HelloWorld".to_string()))
	}
}

struct Display<T>(T);

impl<T> Future for Display<T> where T: Future, T::Item: std::fmt::Display {
	type Item = ();
	type Error = T::Error;
	fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
		let x = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		if x % 2 == 0 {
			match self.0.poll() {
				Ok(Async::Ready(t)) => {
					println!("normal: {}", t);
					Ok(Async::Ready(()))
				},
				Ok(Async::NotReady) => Ok(Async::NotReady),
				Err(e) => Err(e),
			}
		} else {
			let s = try_ready!(self.0.poll());
			println!("clean: {}", s);
			Ok(Async::Ready(()))
		}
	}
}

fn main() {
	let hello = HelloWorld {}.then(|s| {
		println!("{}", s.unwrap());
		tokio::spawn(Display(HelloWorld));
		Ok(())
	});
	tokio::run(hello);
}
