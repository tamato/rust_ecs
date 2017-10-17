#[derive(Debug, PartialEq)]
enum Event {
    Render,
}

trait System {
    fn process(&self);
    fn listening(&self, event: &Event) -> bool;
}

#[derive(Debug)]
struct Renderer {
    who:i32,
    listens_to: Event,
}

impl Renderer {
    fn create() -> Self {
        Renderer {
            who: 0,
            listens_to: Event::Render,
        }
    }
}

impl System for Renderer {
    fn process(&self) {
        println!("Rendering {:?}!", self);
    }

    fn listening(&self, event: &Event) -> bool
    {
        self.listens_to == *event
    }
}

// find a way to pass in a RenderData, which is a datatype that has different properties
type SystemVec<'a> = Vec<Box<System + 'a>>;
struct World<'a> {
    systems: SystemVec<'a>,
    events: Vec<Event>,

    // components

}
impl<'a> World<'a> {
    fn new() -> Self {
        World {
            systems: Vec::new(),
            events: Vec::new(),
        }
    }

    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn add_system<S: System + 'a>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    fn run(&self) {
        for evt in &self.events {
            for sys in &self.systems {
                if sys.listening(evt) {
                    sys.process();
                }
            }
        }
    }

    fn clear(&mut self) {
        self.systems = Vec::new();
        self.events = Vec::new();
    }
}

fn main() {
    let mut w = World::new();

    w.add_event(Event::Render);

    w.add_system(Renderer::create());

    w.run();
    w.clear();
}
