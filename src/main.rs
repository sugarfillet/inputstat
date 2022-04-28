use std::collections::HashMap;
use std::env::Args;
use std::process::exit;
use std::thread;
use evdev::EventType;
use chars::KeyHit;

fn do_help(err : &str) -> ! {

    eprintln!("Please input correctly Bro, Like this \n# sudo chars -k /dev/input/event5 -m /dev/input/event14");
    if err.len() != 0 {

        eprintln!("Detailed Error is {}",err);
    }
    exit(2);
}

fn parse_args(mut list :Args) -> Result<(String,String),&'static str>{
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


fn main() {

    let  args= std::env::args();
    if args.len() != 5 {
        do_help("");
    }
    let (kd,md) = parse_args(args).unwrap_or_else(|err|{ do_help(err);});

    let k_counter = HashMap::new();
    let m_counter = HashMap::new();

    let mut keyboard = KeyHit::new(
        "keyboard".to_string(),
        kd,
        EventType::KEY,
        k_counter);

    let mut mouse = KeyHit::new(
        "mouse".to_string(),
        md,
        EventType::KEY,
        m_counter);

    let _handle1 = thread::spawn( move || {
        let key = &mut keyboard;
        let mut dev =  key.open_device();
        loop {
            // get all inputevents
            for ev in dev.fetch_events().unwrap() {
                key.do_count(ev);
            }
        }
    });

    let _handle2 = thread::spawn( move || {
        let key = &mut mouse;
        let mut dev =  key.open_device();
        loop {
            // get all inputevents
            for ev in dev.fetch_events().unwrap() {
                key.do_count(ev);
            }
        }
    });


    _handle1.join().unwrap();
    _handle2.join().unwrap();


}




