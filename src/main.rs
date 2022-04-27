use std::collections::HashMap;
use std::thread;
use evdev::EventType;
use chars::KeyHit;

fn main() {

 //   let mut args= std::env::args();

    let k_counter = HashMap::new();
    let m_counter = HashMap::new();

    let mut keyboard = KeyHit::new("keyboard".to_string(), "/dev/input/event4".to_string(), EventType::KEY, k_counter);
    let mut mouse = KeyHit::new("mouse".to_string(), "/dev/input/event13".to_string(), EventType::KEY, m_counter);

    //let mut listeners = vec![ keyboard,mouse, ];

   // let key = &mut listeners[0];


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




