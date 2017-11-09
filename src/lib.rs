extern crate rand;

#[derive(Debug, Clone)]
pub struct ComponentCollection {
    pub gfx: Vec<char>,
    pub atk: Vec<i32>,
    pub def: Vec<i32>,
}

impl ComponentCollection {
    pub fn new() -> Self {
        ComponentCollection {
            gfx: Vec::new(),
            atk: Vec::new(),
            def: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeltaCollection {
    pub damage: Vec<i32>,
    // healing: Vec<i32>,
}

impl DeltaCollection {
    pub fn new() -> Self {
        DeltaCollection {
            damage: Vec::new(),
        }
    }
}

pub trait MsgSystem {
    fn process(&self, source_comps: &ComponentCollection, comps_deltas: &mut DeltaCollection);
}

#[derive(Debug, Clone)]
pub struct Renderer {
    pub who: usize,
}

impl MsgSystem for Renderer {
    #[allow(unused_variables)]
    fn process(&self, source_comps: &ComponentCollection, comps_deltas: &mut DeltaCollection) {
        let atk_gfx = &source_comps.gfx[self.who];
        println!("Single {:?}!", atk_gfx);
    }
}

#[derive(Debug, Clone)]
pub struct BasicMeleeAtk {
    pub atk: usize,
    pub def: usize,
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
pub struct World<'a> {
    // list of the different message traits
    pub messages: MsgSystemVec<'a>,

    /// list of unique ids for the entities
    pub ent_list: Vec<usize>,

    pub comps: ComponentCollection,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World {
            messages: Vec::new(),
            ent_list: Vec::new(),
            comps: ComponentCollection::new(),
        }
    }

    pub fn clear(&mut self) {
        self.messages = Vec::new();
    }

    pub fn run(&mut self) {
        let mut deltas_list: Vec<DeltaCollection> = Vec::new();
        let source = self.comps.clone();
        for msg in &self.messages {
            let mut deltas = DeltaCollection::new();
            msg.process(&source, &mut deltas);
            deltas_list.push(deltas);
        }
    }

    pub fn add_message<M>(&mut self, msg: M)
        where M: MsgSystem + 'a
    {
        self.messages.push(Box::new(msg));
    }
}

