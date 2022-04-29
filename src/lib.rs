use std::env::Args;
use std::process::exit;
use std::time::SystemTime;


#[derive(Debug,Copy, Clone)]
pub struct TheEvent(SystemTime,u16);
impl TheEvent{
    pub fn new(time:SystemTime,code:u16) -> TheEvent {
        TheEvent(time,code)
    }
}













/// helper
///
pub fn do_help(err : &str) -> ! {

    eprintln!("Please input correctly Bro, Like this \n# sudo chars -k /dev/input/event5 -m /dev/input/event14");
    if err.len() != 0 {

        eprintln!("Detailed Error is {}",err);
    }
    exit(2);
}

pub fn parse_args(mut list :Args) -> Result<(String,String),&'static str>{
    list.next();
    list.next();

    let x = match list.next() {
        Some(x) => x,
        None => return Err("get keyboard failed")
    };

    list.next();

    let y = match list.next() {
        Some(x) => x,
        None => return Err("get mouse failed")
    };

    Ok((x,y))
}

