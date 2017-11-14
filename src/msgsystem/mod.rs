
use components::{ComponentCollection, DeltaCollection};
use rng_range;

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

impl MsgSystem for BasicMeleeAtk {
    fn process(&self, source_comps: &ComponentCollection, comps_deltas: &mut DeltaCollection) {
        let atk = &source_comps.atk[self.atk];
        let atk_dam = rng_range(0, *atk);

        let def = &source_comps.def[self.def];
        let def_dam = rng_range(0, *def);
        let total = ::std::cmp::max(0, atk_dam - def_dam);
        comps_deltas.damage.push(total);

        let atk_gfx = &source_comps.gfx[self.atk];
        let def_gfx = &source_comps.gfx[self.def];
        println!("{:?} -> {:?} | {:?} - {:?} | {:?} ", atk_gfx, def_gfx, atk_dam, def_dam, total);
    }
}



