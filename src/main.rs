extern crate rand;

trait MsgSystem {
    fn process(&self, source_comps: &ComponentCollection, comps_deltas: &mut DeltaCollection);
}

#[derive(Debug, Clone)]
struct Renderer {
    who: usize,
}

impl MsgSystem for Renderer {
    #[allow(unused_variables)]
    fn process(&self, source_comps: &ComponentCollection, comps_deltas: &mut DeltaCollection) {
        let atk_gfx = &source_comps.gfx[self.who];
        println!("Single {:?}!", atk_gfx);
    }
}

#[derive(Debug, Clone)]
struct BasicMeleeAtk {
    atk: usize,
    def: usize,
}

fn rng_range(start: i32, end: i32) -> i32 {
    let range = (end - start) as u32;
    let offset = start as u32;
    ((rand::random::<u32>() % range) + offset) as i32
}

impl MsgSystem for BasicMeleeAtk {
    fn process(&self, source_comps: &ComponentCollection, comps_deltas: &mut DeltaCollection) {
        let atk = &source_comps.atk[self.atk];
        let atk_dam = rng_range(0, *atk);

        let def = &source_comps.def[self.def];
        let def_dam = rng_range(0, *def);
        let total = std::cmp::max(0, atk_dam - def_dam);
        comps_deltas.damage.push(total);

        let atk_gfx = &source_comps.gfx[self.atk];
        let def_gfx = &source_comps.gfx[self.def];
        println!("{:?} -> {:?} | {:?} - {:?} | {:?} ", atk_gfx, def_gfx, atk_dam, def_dam, total);
    }
}

type MsgSystemVec<'a> = Vec<Box<MsgSystem + 'a>>;
struct World<'a> {
    // list of the different message traits
    messages: MsgSystemVec<'a>,

    /// list of unique ids for the entities
    ent_list: Vec<usize>,

    comps: ComponentCollection,
}
impl<'a> World<'a> {
    fn new() -> Self {
        World {
            messages: Vec::new(),
            ent_list: Vec::new(),
            comps: ComponentCollection::new(),
        }
    }

    fn clear(&mut self) {
        self.messages = Vec::new();
    }

    fn run(&mut self) {
        let mut deltas_list: Vec<DeltaCollection> = Vec::new();
        let source = self.comps.clone();
        for msg in &self.messages {
            let mut deltas = DeltaCollection::new();
            msg.process(&source, &mut deltas);
            deltas_list.push(deltas);
        }
    }

    fn add_message<M>(&mut self, msg: M)
        where M: MsgSystem + 'a
    {
        self.messages.push(Box::new(msg));
    }
}

#[derive(Debug, Clone)]
struct ComponentCollection {
    gfx: Vec<char>,
    atk: Vec<i32>,
    def: Vec<i32>,
}

impl ComponentCollection {
    fn new() -> Self {
        ComponentCollection {
            gfx: Vec::new(),
            atk: Vec::new(),
            def: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct DeltaCollection {
    damage: Vec<i32>,
    // healing: Vec<i32>,
}

impl DeltaCollection {
    fn new() -> Self {
        DeltaCollection {
            damage: Vec::new(),
        }
    }
}

fn main() {
    let mut w = World::new();
    let mut comps = ComponentCollection::new();

    w.ent_list.push(0);
    comps.gfx.push('@');
    comps.atk.push(15);
    comps.def.push(10);
    
    w.ent_list.push(1);
    comps.gfx.push('B');
    comps.atk.push(15);
    comps.def.push(10);
    
    w.comps = comps.clone();

    w.add_message(Renderer{who:0});
    w.add_message(Renderer{who:1});

    w.add_message(BasicMeleeAtk{atk:0, def:1});
    w.add_message(BasicMeleeAtk{atk:1, def:0});
    w.run();
    w.clear();
}

//  https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust
//  https://users.rust-lang.org/t/how-can-i-correctly-implement-observer-pattern-in-rust/6058/7


