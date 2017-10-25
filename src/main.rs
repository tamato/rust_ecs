
trait MsgSystem {
    fn process(&self, world: &World);
}

#[derive(Debug)]
struct Renderer {
    who: usize,
}

impl MsgSystem for Renderer {
    fn process(&self, world: &World) {
        let atk_gfx = &world.gfx_componets[self.who];
        println!("Single {:?}!", atk_gfx);
    }
}

#[derive(Debug)]
struct BasicMeleeAtk {
    atk: usize,
    def: usize,
}

impl MsgSystem for BasicMeleeAtk {
    fn process(&self, world: &World) {
        let atk_gfx = &world.gfx_componets[self.atk];
        let def_gfx = &world.gfx_componets[self.def];
        println!("{:?} is attacking {:?}!", atk_gfx, def_gfx);
    }
}

type MsgSystemVec<'a> = Vec<Box<MsgSystem + 'a>>;
struct World<'a> {
    // list of the different message traits
    messages: MsgSystemVec<'a>,

    /// list of unique ids for the entities
    ent_list: Vec<usize>,

    // components
    gfx_componets: Vec<char>,
}

impl<'a> World<'a> {
    fn new() -> Self {
        World {
            messages: Vec::new(),
            ent_list: Vec::new(),
            gfx_componets: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.messages = Vec::new();
    }

    // fn get_next_id(&mut self) -> usize {
    //     let v = self.ent_list.len();
    //     self.ent_list.push(v);
    //     v
    // }

    fn run(&mut self) {
        for msg in &self.messages {
            msg.process(self);
        }
    }

    fn add_message<M>(&mut self, msg: M)
        where M: MsgSystem + 'a
    {
        self.messages.push(Box::new(msg));
    }
}

fn main() {
    let mut w = World::new();

    w.ent_list.push(0);
    w.gfx_componets.push('@');
    w.ent_list.push(1);
    w.gfx_componets.push('B');

    w.add_message(Renderer{who:0});
    w.add_message(Renderer{who:1});

    w.add_message(BasicMeleeAtk{atk:0, def:1});
    w.add_message(BasicMeleeAtk{atk:1, def:0});
    w.run();
    w.clear();
}



