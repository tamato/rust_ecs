
extern crate rust_ecs;
use rust_ecs::world::*;
use rust_ecs::msgsystem::*;
use rust_ecs::components::*;

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


