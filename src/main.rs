
#[derive(Debug)]
struct Point2 {
    x:i32,
    y:i32,
}

#[derive(Debug)]
struct World {
    shutDown: bool,

    // entities
    things:Vec<usize>,

    // components
    positions:Vec<Point2>,

    // systems / events
    systems_to_run: Vec<SystemTypes>,
    next_cycle_systems: Vec<SystemTypes>,

    // resources
    delta_time: f32,
    // total_time: f32,
}

impl World {
    fn new() -> Self {
        World {
            shutDown: false,
            things: vec![0],
            positions: vec![Point2{x:0, y:0}],
            systems_to_run: Vec::new(),
            next_cycle_systems:Vec::new(),
            delta_time: 0.001,
        }
    }

    fn run_systems(&mut self, systems: &mut Systems) {
        println!("start run_systems");
        let tmp_systems: Vec<SystemTypes> = self.systems_to_run.drain(..).collect();
        for sys_type in tmp_systems {
            systems.test(self, sys_type)
        }
        
        self.systems_to_run = self.next_cycle_systems.drain(..).collect();
        self.next_cycle_systems = Vec::new();
        println!("end run_systems");
    }

    fn enqueue_system(&mut self, sys_type: SystemTypes) {
        self.next_cycle_systems.push(sys_type);
    }

    fn intial_systems(&mut self, sys_type: SystemTypes) {
        self.systems_to_run.push(sys_type);
    }
}

#[derive(Debug)]
enum SystemTypes {
    PlayerInputType,
    CloseAppType,
    WalkType,
}

struct Systems {
    player_input: PlayerInput,
    walk: Walk,
    close: CloseApp,
}
impl Systems {
    fn new() -> Self {
        Systems {
            player_input: PlayerInput{},
            walk: Walk{},
            close: CloseApp{},
        }
    }
    fn test(&mut self, world: &mut World, sys_type: SystemTypes) {
        println!("running test...");
        match sys_type {
            SystemTypes::PlayerInputType => self.player_input.process(world),
            SystemTypes::CloseAppType => self.close.process(world),
            SystemTypes::WalkType => self.walk.process(world),
        }
    }
}

trait System {
    fn process(&self, world: &mut World);
}

#[derive(Debug)]
struct PlayerInput;
impl System for PlayerInput {
    fn process(&self, world: &mut World) {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        let trimed = line.trim();
        println!("You entered: {} ---", trimed);

        // use std::io::Read;
        // let input: Option<i32> = std::io::stdin()
        //     .bytes() 
        //     .next()
        //     .and_then(|result| result.ok())
        //     .map(|byte| byte as i32);
        // println!("Byte: {:?}", input);
        // world.enqueue_system(SystemTypes::PlayerInputType);

        match trimed.as_ref() {
            "qq"  => world.enqueue_system(SystemTypes::CloseAppType),
            "exit"=> world.enqueue_system(SystemTypes::CloseAppType),
            "2"|"4"|"6"|"8" => {
                println!("You pressed a direction");
                world.enqueue_system(SystemTypes::PlayerInputType);
            },
            _ => world.enqueue_system(SystemTypes::PlayerInputType),
        };
        
    }
}

#[derive(Debug)]
struct CloseApp;
impl System for CloseApp {
    fn process(&self, world: &mut World) {
        world.shutDown = true;
        println!("Shutting it down!");
    }
}

#[derive(Debug)]
struct Walk {
    ids: Vec<usize>,
};
impl System for Walk {
    fn process(&self, world: &mut World) {
        // this needs to know WHO is trying to walk
        println!("do walk...");
    }
}

fn main() {
    let mut w = World::new();
    let mut s = Systems::new();
    
    w.intial_systems(SystemTypes::PlayerInputType);

    while w.shutDown == false {
        println!("main loop");
        w.run_systems(&mut s);
    }
}


/*
    player update {
        input match
        move
        menu
            menus
            abilities
            inventory
            help
    }
*/

