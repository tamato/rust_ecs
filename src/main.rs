use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
enum MessageType {
    Render,
    BasicMelee,
}

trait MsgSystem {
    fn process(&self, who: usize, world: &World);
}

#[derive(Debug)]
struct Renderer;

impl Renderer {
    fn new() -> Self {
        Renderer {
        }
    }
}

impl MsgSystem for Renderer {
    fn process(&self, who: usize, world: &World) {
        let gfx = &world.gfx_componets[who];
        println!("Rendering {:?}!", gfx);
    }
}

#[derive(Debug)]
struct BasicMeleeAtk {
    def:Vec<usize>,
}

impl BasicMeleeAtk {
    fn new() -> Self {
        BasicMeleeAtk {
            def: Vec::new(),
        }
    }
}

impl MsgSystem for BasicMeleeAtk {
    fn process(&self, who: usize, world: &World) {
        let def_id = world.target_components[who].clone();
        let def_gfx = &world.gfx_componets[*def_id];

        let atk_gfx = &world.gfx_componets[who];
        println!("{:?} is attacking {:?}!", atk_gfx, def_gfx);
    }
}

type MsgSystemVec<'a> = Vec<Box<MsgSystem + 'a>>;
struct World<'a> {
    // list of the different message traits
    systems: MsgSystemVec<'a>,

    // pairings between a message system and which entities to act on
    msg_who: HashMap<MessageType, Vec<usize>>,

    /// list of unique ids for the entities
    ent_list: Vec<usize>,

    // components
    gfx_componets: Vec<char>,
    target_components: Vec<usize>,
}

impl<'a> World<'a> {
    fn new() -> Self {
        World {
            systems: Vec::new(),
            ent_list: Vec::new(),
            msg_who: HashMap::new(),
            gfx_componets: Vec::new(),
            target_components: Vec::new(),
        }
    }

    fn add_system<S: MsgSystem + 'a>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    fn run(&self) {
        for (msg, ent_vec) in &self.msg_who {
            let sys = &self.systems[ msg.clone() as usize ];
            for ent in ent_vec.iter() {
                sys.process(*ent, self);
            }
        }   
    }

    fn clear(&mut self) {
        self.systems = Vec::new();
    }

    // fn get_next_id(&mut self) -> usize {
    //     let v = self.ent_list.len();
    //     self.ent_list.push(v);
    //     v
    // }

    fn add_msg(&mut self, msg: MessageType, who: usize) {
        self.msg_who
            .entry(msg)             // get the value of the passed in key
            .or_insert(Vec::new())  // if it does not exist, create a new one
            .push(who);             // push value into the vec at this key
    }
}

fn main() {
    let mut w = World::new();

    w.add_system(Renderer::new());
    w.add_system(BasicMeleeAtk::new());

    w.ent_list.push(0);
    w.gfx_componets.push('@');
    w.ent_list.push(1);
    w.gfx_componets.push('B');

    w.add_msg(MessageType::Render, 0);
    w.add_msg(MessageType::Render, 1);
    w.run();
    w.clear();
}
