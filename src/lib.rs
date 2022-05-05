//! inputstat
//!
//! `inputstat` is a vmstat-like tool which monitors keyboard and mouse events on linux.


use std::collections::HashMap;
use std::env::Args;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use evdev::{Device, InputEventKind, RelativeAxisType};





/// input event
#[derive(Debug,Copy, Clone)]
pub struct TheEvent(SystemTime,u16,bool);
impl TheEvent{
    pub fn new(time:SystemTime, code:u16, x1: bool ) -> TheEvent {
        TheEvent(time,code,x1)
    }
    pub fn code(&self) -> u16 {
        self.1
    }
    pub fn typp(&self) -> String{
        if self.2 {
            String::from("KEYBOARD")
        } else {

            String::from("MOUSE")
        }
    }
}


/// mouse or keyboard
pub struct Dev(String,bool);
impl Dev{
    pub fn new(s:String,t:bool) -> Dev{
        Dev(s,t)
    }
}


/// event handling routine

pub fn do_send (k_or_m :Dev, tx: Sender<TheEvent>){

    let mut dev = Device::open(k_or_m.0 ).unwrap();
    loop {
        for ev in dev.fetch_events().unwrap() {
            match ev.kind(){
                InputEventKind::Key(_) if ev.value() == 0 => {
                    tx.send( TheEvent::new(ev.timestamp(),ev.code(),k_or_m.1)).unwrap();
                },
                InputEventKind::RelAxis(ww) =>
                    if ww == RelativeAxisType::REL_WHEEL {
                        tx.send( TheEvent::new(ev.timestamp(),0xffff, false)).unwrap();
                    },
                _ => {}
            };
        }
    }

}

/// output routine

pub fn do_timer(int:u64,top:u64 ,q1:Arc<Mutex<Vec<TheEvent>>>){
    loop {
        sleep(Duration::new(int , 0));
        let q = q1.lock().unwrap();
        let mut k = 0 ;
        let mut m = 0;
        let mut k_map = HashMap::new();
        let mut m_map = HashMap::new();
        let len = q.len();
        for x in q.iter(){
            if x.typp() == "KEYBOARD".to_string() {
                k +=1;
                let count = k_map.entry(x.code()).or_insert(0);
                *count += 1;
            }else if x.typp() == "MOUSE".to_string(){
                m +=1;
                let count = m_map.entry(x.code()).or_insert(0);
                *count += 1;
            }

        }
        //q.clear();
        println!(">>  summary counts {} ## keyboard counts {} ## mouse counts {}",len,k,m);
        print!(">> ");

        let mut sorted :Vec<_> = k_map.iter().collect();
        sorted.sort_by_key(|a| a.1);
        sorted.reverse();


        for (k,v) in sorted.iter().enumerate(){
            if k  == top as usize {
                break;
            }
            print!(" {:?} -> {} ## ",evdev::Key(*(*v).0),*(*v).1);
        }

        println!();

        print!(">> ");

        let mut sorted :Vec<_> = m_map.iter().collect();
        sorted.sort_by_key(|a| a.1);
        sorted.reverse();
        for (k,v) in sorted{
            if *k == 0xffff {
                print!(" WHEEL -> {} ## ",v);
            } else {
                print!(" {:?} -> {} ## ",evdev::Key(*k),*v);
            }
        }
        println!();

        println!();
    }
}









/// cmdline hinter
///
pub fn do_help(err : &str) -> ! {

    eprintln!("Please input correctly Bro, Like this \n# sudo inputstat -k /dev/input/event5 -m /dev/input/event14 5 ");
    if err.len() != 0 {

        eprintln!("Detailed Error is {}",err);
    }
    exit(2);
}


/// arguments parser
///
pub fn parse_args(mut list :Args) -> Result<(String,String,u64),&'static str>{
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

    let z = match list.next() {
        Some(x) => x.parse().unwrap(),
        None => return Err("get mouse failed")
    };

    Ok((x,y,z))
}

