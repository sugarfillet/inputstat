extern crate core;


use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use inputstat::{Dev, do_help, do_send, do_timer, parse_args, };





fn main() {

    let  args= std::env::args();
    if args.len() != 6 {
        do_help("");
    }
    let (kd,md,int) = parse_args(args).unwrap_or_else(|err|{ do_help(err);});

    let kd= Dev::new(kd,true);
    let md = Dev::new(md,false);

    let (tx,rx) = mpsc::channel();
    let tx_m = tx.clone();

    thread::spawn( move || {
        do_send(kd,tx);
    });

    thread::spawn( move || {
        do_send(md,tx_m);
    });



    let queue = Arc::new(Mutex::new(Vec::new()));

    // push the received events push to queue
    let q0 = Arc::clone(&queue);
    thread::spawn(move || {
        for x in rx{
            let mut q = q0.lock().unwrap();
            q.push(x);
        }
    });

    // show the queue every <int> seconds
    let q1 = Arc::clone(&queue);
    do_timer(int,5,q1);

}





