// accordng to some the Observer pattern is too OOP for Rust
//  going to try an event system
//  push all events in one loop
//  process all of them

// for something like combat,
//  attack loc -> event system
//  dont think too much about it, just try it all in events and think about it at the end.

use std::any::Any;
use std::collections::HashMap;

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

type ComponentMap = HashMap<usize, Box<Any>>;
struct ComponentList {
    components: HashMap< ComponentType, ComponentMap >,
}

impl ComponentList {
    fn new() -> Self {
        ComponentList {
            components: HashMap::new(),
        }
    }

    fn add(&mut self, entity: usize, c_type: ComponentType, val: char ) {
        match self.components.get(&c_type) {
            Some(comp) => comp.insert(entity, Box::new(val)),
            None => {
                let first: ComponentMap = HashMap::new();
                first.insert(entity, Box::new(val));
                self.components.insert(c_type, first);
            },
        };
    }
}

fn main() {
    let mut events = EventList::new();
    let mut comps = ComponentList::new();
    let mut ents = Entities::new();

    let ent0 = ents.create();
    let p0 = Print{x:ent0};

    let ent1 = ents.create();
    let p1 = Print{x:ent1};

    comps.add(ent0, ComponentType::Gfx, 'G');
    comps.add(ent1, ComponentType::Gfx, 'Z');

    events.add(p0);
    events.add(p1);
    events.run(&comps);
    events.clear();

    let p1 = Print{x:ent1};
    events.add(p1);
    events.run(&comps);
    events.clear();
}

struct Entities {
    next_free: usize,
    list: Vec<usize>,
}

impl Entities {
    fn new() -> Self {
        Entities {
            next_free: 0,
            list: Vec::new(),
        }
    }

    fn create(&mut self) -> usize {
        let result = self.next_free;
        self.list.push(result);
        self.next_free = self.list.len();
        result
    }
}

trait Event {
    fn run(&self, comp: &ComponentList);
}

struct Print {
    x: usize,
}

impl Event for Print {
    fn run(&self, comp: &ComponentList) {
        get_val(&self.x, comp, |val| {
            println!("values! {}, {}", self.x, val);
        });
    }
}

fn get_val<F>(id: &usize, comp: &ComponentList, func: F)
    where F: Fn(char)
{
    match comp.components.get(&ComponentType::Gfx) {
        Some(gfx_comp) => match gfx_comp.get(&id) {
            Some(value_any) => match value_any.downcast_ref::<char>() {
                /***                              *******/
                Some(val) => func( val ),
                /***                              *******/
                None => panic!("val in Print is not of type char!"),
            },
            None => panic!("Print is missing self.x value"),
        },
        None => panic!("ComponentType::Gfx has not been added to ComponentList"),
    }
}

#[derive(Eq, PartialEq, Hash)]
enum ComponentType {
    Gfx,
}
