use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug,Clone,PartialEq,Eq)]
//Debug is for checking detail
//enable to clone in enum
//eq / Partialeq for compare && contrast purpose
pub enum Target_Ip{
    Single(IpAddr),
    CIDR(Ipnet),
}

impl ScanTarget{
    pub fn parse(input:&str) -> Result<Self,String>{
        let text = input.trim();
        //let input be clear 
        
        //Here is to check the IP address is CIDR or nah
        if text.contains('/') {
            .map(Target_Ip::CIDR) // let it go into the CIDR enum
            .map_err((|_| format!("[!]  '{}' Not is not a valid IP Address!!!!",text))


        }else{//means that it must be a single ip
                IpAddr::from_str(text)
                    .map(ScanTarget::Single)//let it be a single address
                    .map_err(|_| format!("[!] '{}' is not a valid text!!!",text))
        }
    }
}