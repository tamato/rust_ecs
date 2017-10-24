use std::collections::HashMap;
use std::ops::Index;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
enum Messages {
    Render,
}

trait System {
    fn process(&self, world: &World);
    fn process_ent(&self, who: usize, world: &World);
}

#[derive(Debug)]
struct Renderer {
    who:Vec<usize>,
}

impl Renderer {
    fn create() -> Self {
        Renderer {
            who: Vec::new(),
        }
    }
}

impl System for Renderer {
    fn process(&self, world: &World) {
        for ent in &world.ent_list {
            let gfx = &world.gfx_componets[*ent];
            println!("Rendering {:?}!", gfx);
        }
    }
    fn process_ent(&self, who: usize, world: &World)
    {
        let gfx = &world.gfx_componets[who];
        println!("Rendering {:?}!", gfx);
    }
}

// find a way to pass in a RenderData, which is a datatype that has different properties
type SystemVec<'a> = Vec<Box<System + 'a>>;
struct World<'a> {
    systems: SystemVec<'a>,

    // components
    gfx_componets: Vec<char>,

    /// list of unique ids for the entities
    ent_list: Vec<usize>,


    // list of messages 
    msg_who: HashMap<Messages, Vec<usize>>,
}

impl<'a> World<'a> {
    fn new() -> Self {
        World {
            systems: Vec::new(),
            gfx_componets: Vec::new(),
            ent_list: Vec::new(),
            msg_who: HashMap::new(),
        }
    }

    fn add_system<S: System + 'a>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    fn run(&self) {
        for sys in &self.systems {
            sys.process(self);
        }
    }

    fn run_msg(&self) {
        for (msg, ent_vec) in &self.msg_who {
            let sys = &self.systems[ msg.clone() as usize ];
            for ent in ent_vec.iter() {
                sys.process_ent(*ent, self);
            }
        }   
    }

    fn clear(&mut self) {
        self.systems = Vec::new();
    }

    fn get_next_id(&mut self) -> usize {
        let v = self.ent_list.len();
        self.ent_list.push(v);
        v
    }

    fn add_msg(&mut self, msg: Messages, who: usize) {
        self.msg_who.entry(msg).or_insert(Vec::new()).push(who);
    }
}

fn main() {
    let mut w = World::new();
    w.add_system(Renderer::create());

    w.run();
    w.clear();

    w.add_system(Renderer::create());
    w.ent_list.push(0);
    w.gfx_componets.push('@');
    w.ent_list.push(1);
    w.gfx_componets.push('#');

    w.add_msg(Messages::Render, 0);
    w.add_msg(Messages::Render, 1);
    w.run_msg();
}


trait Componet {}

#[derive(Debug)]
struct Renderable {
    gfx: char,
}

impl Componet for Renderable {

}

// https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
