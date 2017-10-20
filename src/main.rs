// accordng to some the Observer pattern is too OOP for Rust
//  going to try an event system
//  push all events in one loop
//  process all of them

// for something like combat,
//  attack loc -> event system
//  dont think too much about it, just try it all in events and think about it at the end.

type Observer = Box<Fn()>;

// subjects
struct Subject {
    obs: Vec<Observer>,
}

fn main() {
    let a = ||{ println!("hello"); };
    let b = ||{ println!(" world"); };


    let mut s = Subject {
        obs: vec![Box::new(a), Box::new(b)],
    };

    for x in &s.obs {
        x();
    }

    let ez = 999;
    s.obs.push(Box::new( move ||{
        println!("this is not {}", ez);
    }));

    for x in &s.obs {
        x();
    }
}

