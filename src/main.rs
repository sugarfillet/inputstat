use std::collections::HashMap;
use std::env::Args;
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::SystemTime;
use evdev::{Device, EventType, InputEventKind, RelativeAxisType};
use chars::{do_help, parse_args, TheEvent};





fn main() {

    let  args= std::env::args();
    if args.len() != 5 {
        do_help("");
    }
    let (kd,md) = parse_args(args).unwrap_or_else(|err|{ do_help(err);});

    let (tx,rx) = mpsc::channel();
    let tx_m = tx.clone();

    thread::spawn( move || {
        do_send(kd,tx);
    });

    thread::spawn( move || {
        do_send(md,tx_m);
    });

    for x in rx{
        println!("{:?}",x);
    }

}

fn do_send (kd:String, tx: Sender<TheEvent>){

    let mut dev = Device::open(kd).unwrap();
    loop {
        for ev in dev.fetch_events().unwrap() {
            match ev.kind(){
                InputEventKind::Key(x) if ev.value() == 0 => {
                    tx.send( TheEvent::new(ev.timestamp(),ev.code())).unwrap();
                },
                InputEventKind::RelAxis(ww) =>
                    if ww == RelativeAxisType::REL_WHEEL {
                        tx.send( TheEvent::new(ev.timestamp(),0xffff)).unwrap();
                    },
                _ => {}
            };
        }
    }

}



