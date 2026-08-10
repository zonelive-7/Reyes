mod ip_proc;
mod port_proc;
mod scanner;
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::net::SocketAddr;
use std::time::Duration;


enum ScanResult{
	Open,
	Close,
	Filtered,
}


async fn scan_port(addr:SocketAddr , timeout_ms:u64) -> ScanResult{
	//means parameter is a address and set a u64 type of time, return ScanResult
	match timeout(Duration::from_millis(timeout_ms),TcpStream::connect(addr)).await{
		Ok(Ok(_)) => ScanResult::Open,
		Ok(Err(_)) => ScanResult::Close,
		Err(_) => ScanResult::Filtered,
	}
}



#[tokio::main]
async fn main() {
	let ip : std::net::IpAddr = ("192.168.100.9").parse().unwrap();

	for port in 20..=25{
		let addr = SocketAddr::new(ip,port);
		let status = scan_port(addr , 500).await;


		match status{
			ScanResult::Open => println!("Port {} : OPENED",port),
			ScanResult::Close => println!("Port {} : CLOSED ",port),
			ScanResult::Filtered => println!("Port {} : Filterd",port),

		}

	}
	
}
