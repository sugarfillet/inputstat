use std::collections::HashMap;
use evdev::{Device, EventType, InputEvent};
use evdev::InputEventKind::Key;

pub struct KeyHit {
    name : String ,
    device : String,
    types : EventType,
    count : HashMap<evdev::Key,u64> ,
}

impl KeyHit {

    pub fn new(name:String,device:String,types:EventType,count:HashMap<evdev::Key,u64>) -> KeyHit {
        KeyHit { name, device, types, count, }
    }

    pub fn do_count(&mut self, ev: InputEvent) {
        if ev.event_type() == self.types && ev.value() == 0 {
            if let Key(k) = ev.kind() {
                let count = self.count.entry(k).or_insert(0);
                *count += 1;
                self.print_counter();
            }

        }
    }
    pub fn open_device(&self) -> Device {
        Device::open(&self.device).unwrap()
    }

    fn print_counter(&self){

        println!("XXXXXXXX {} XXXXXXXXXX",self.name);
        for (k,v) in &self.count{
            println!("key is {:?} counter is {}",k,v);
        }
        println!();
    }

}


