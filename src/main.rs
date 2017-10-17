#[derive(Debug)]
enum SystemTypes {
    Renderer,
}

#[derive(Debug)]
enum ComponetTypes {
    Renderable,
}

trait System {
    fn register_ent(&mut self, ent: usize);
    fn process(&self, world: &World);
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
    fn register_ent(&mut self, ent: usize) {
        self.who.push(ent);
    }

    fn process(&self, world: &World) {
        for ent in &self.who {
            let gfx = &world.gfx_componets[*ent];
            println!("Rendering {:?}!", gfx);
        }
    }
}

// find a way to pass in a RenderData, which is a datatype that has different properties
type SystemVec<'a> = Vec<Box<System + 'a>>;
type ComponetVec<'a> = Vec<Box<Componet + 'a>>;
struct World<'a> {
    systems: SystemVec<'a>,

    // components
    gfx_componets: ComponetVec<'a>,

    /// list of unique ids for the entities
    ent_list: Vec<usize>,
}

impl<'a> World<'a> {
    fn new() -> Self {
        World {
            systems: Vec::new(),
            gfx_componets: Vec::new(),
            ent_list: Vec::new(),
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

    fn clear(&mut self) {
        self.systems = Vec::new();
    }

    fn get_next_id(&mut self) -> usize {
        let v = self.ent_list.len();
        self.ent_list.push(v);
        v
    }

    fn register_ent(&mut self, systems: Vec<SystemTypes>) {

    }
}

fn createEnt(world: &mut World) {
    let ent_id = world.get_next_id();

    // components used by ent
    let gfx = Renderable{ comp_type: ComponetTypes::Renderable, gfx: '@' };

    // who to register to
    let register_to = vec![SystemTypes::Renderer];

    
}

fn main() {
    let mut w = World::new();
    w.add_system(Renderer::create());

    w.run();
    w.clear();
}


trait Componet {}

#[derive(Debug)]
struct Renderable {
    comp_type: ComponetTypes,
    gfx: char,
}

impl Componet for Renderable {

}

// https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
