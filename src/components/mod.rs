
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
