use std::num::ParseIntError;

//let port have a rules
#[derive(Debug, PartialEq)] 
enum PortRule{
	SinglePort(u16),
	RangePort(u16,u16),
}




//做一个range 和 single 分隔的func
//一个是 “-” 分割为range
//else 就是port
//顺便检查有没有乱输入      