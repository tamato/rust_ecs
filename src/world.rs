
use components::{ComponentCollection, DeltaCollection};
use msgsystem::MsgSystem;

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

