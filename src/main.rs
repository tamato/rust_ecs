// accordng to some the Observer pattern is too OOP for Rust
//  going to try an event system
//  push all events in one loop
//  process all of them

// for something like combat,
//  attack loc -> event system
//  

struct EventList<'a> {
    list: Vec<Box<Event + 'a>>,
}

impl<'a> EventList<'a> {
    fn new() -> Self {
        EventList {
            list: Vec::new(),
        }
    }

    fn add<E>(&mut self, event: E )
        where E: Event + 'a 
    {
        self.list.push(Box::new(event));
    }

    fn run(&self, comp: &ComponentList) {
        for evt in &self.list {
            evt.run(comp);
        }        
    }

    fn clear(&mut self) {
        self.list.clear();
    }
}


struct ComponentList {
    gfx: Vec<char>,
}

impl ComponentList {
    fn new() -> Self {
        ComponentList {
            gfx: Vec::new(),
        }
    }

    fn add(&mut self, entity: usize, c_type: ComponentType, val: char ) {
        match c_type {
            ComponentType::Gfx => self.gfx.push(val),
        }
    }
}

fn main() {
    let mut events = EventList::new();
    let mut comps = ComponentList::new();

    comps.add(0, ComponentType::Gfx, 'G');

    let p = Print{x:0};
    events.add(p);
    events.run(&comps);
    events.clear();
}

trait Event {
    fn run(&self, comp: &ComponentList);
}

struct Print {
    x: usize,
}

impl Event for Print {
    fn run(&self, comp: &ComponentList) {
        let val = &comp.gfx[self.x];
        println!("values! {}, {}", self.x, val);
    }
}

enum ComponentType {
    Gfx,
}

/*
https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust
https://users.rust-lang.org/t/how-can-i-correctly-implement-observer-pattern-in-rust/6058/7

pub trait Aggregate<TEvent>: Default {
  fn apply(&mut self, e: &TEvent);
}

#[allow(non_camel_case_types)]
pub enum Event {
 COUNTER_DECREASED { count: i32, amount: i32 },
 COUNTER_INCREASED { count: i32, amount: i32 },
}

pub struct Counter {
 count: i32,
}

impl Default for Counter {
 fn default() -> Self {
   Counter {
     count: 0
   }
 }
}

impl Aggregate<Event> for Counter {
 fn apply(&mut self, e: &Event) {
   match *e {
     Event::COUNTER_DECREASED { count, .. } => self.count = count,
     Event::COUNTER_INCREASED { count, .. } => self.count = count,
   }
 }
}

impl Counter {
 pub fn decrease(&self, amount: i32) -> [Event; 1] {
   [Event::COUNTER_DECREASED { count: self.count - amount, amount: amount }]
 }

 pub fn increase(&self, amount: i32) -> [Event; 1] {
   [Event::COUNTER_INCREASED { count: self.count + amount, amount: amount }]
 }
}


*/
