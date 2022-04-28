use std::collections::HashMap;
use evdev::{Device, EventType, InputEvent,  RelativeAxisType};
use evdev::InputEventKind::{Key, RelAxis};


pub struct KeyHit {
    name : String ,
    device : String,
    types : EventType,
    count : HashMap<u16,u64> ,
}

impl KeyHit {

    pub fn new(name:String,device:String,types:EventType,count:HashMap<u16,u64>) -> KeyHit {
        KeyHit { name, device, types, count, }
    }

    pub fn do_count(&mut self, ev: InputEvent) {
        // YYYYYYYYYYYYY RelAxis(REL_WHEEL_HI_RES)
        // https://patchwork.kernel.org/project/linux-input/patch/20181121152712.6770-8-benjamin.tissoires@redhat.com/

         match ev.kind(){
             // deal keyboard and mouse keys
            Key(kk) if ev.value() == 0  => {
                self.add_counter(kk.code())
            },
             // deal mouse wheel roll
            RelAxis(ww) =>
                if ww == RelativeAxisType::REL_WHEEL {
                    self.add_counter(0xffff)
            },
            _ => {},
        };
    }
    pub fn open_device(&self) -> Device {
        Device::open(&self.device).unwrap()
    }

    fn add_counter(&mut self, k : u16 ){

        // increase
        let count = self.count.entry(k).or_insert(0);
        *count += 1;

        // summary
        let mut cc: u64 = 0 ;
        println!("XXXXXXXX {} XXXXXXXXXX",self.name);

        for (k,v) in &self.count{
            if 0xffff  == *k {
                println!("key is REL_WHEEL counter is {}",v);
            } else {
                println!("key is {:?} counter is {}", evdev::Key::new(*k), v);
            }
                cc += v;
        };

        println!("XXXXXXXX {} XXXXXXXXXX",cc);
        println!();
    }

}


