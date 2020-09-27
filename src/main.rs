use std::io;

#[derive(Debug)]
struct Entity<'a> {
  id: usize,
  x: usize,
  y: usize,
  typ_k: &'a str,
  del: bool,
  utf_art: &'a str,
}

#[derive(Debug)]
struct PlayerInput {
  up: bool,
  down: bool,
  right: bool,
  left: bool,
}

struct World<'a> {
  t: f32,
  entities: Vec<Entity<'a>>,
  entity_typ_order: Vec<&'a str>,
  canvas_buf: Vec<&'a str>,
}

impl World<'_> {
  fn new(entity_typ_order: Vec<&str>) -> World {

    // define entities pool
    let n = 2;
    let mut entities: Vec<Entity> = Vec::with_capacity(n);

    // fill the pool
    for i in 0..n {
      let entity = Entity {
        id: i,
        x: if i % 2 == 0 {3} else {5},
        y: 0,
        typ_k: if i % 2 == 0 {"thing"} else {"player"},
        utf_art: if i % 2 == 0 {"/"} else {"@"},
        del: true,
      };
      entities.push(entity)
    }

    // initialize canvas buffer  
    let canvas_w = 20;
    let mut canvas_buf: Vec<&str> = Vec::with_capacity(canvas_w);
    for _i in 0..canvas_w {
      canvas_buf.push(" ");
    }

    World {
      t: 0.0,
      entities: entities,
      entity_typ_order: entity_typ_order,
      canvas_buf: canvas_buf,
    }
  }
  
  fn tick(&mut self) {
        
    // iterate over world entities types by paint order
    let typ_k_vec = self
      .entity_typ_order
      .iter()
      .map(|&typ_k| typ_k)
      .collect::<Vec<_>>()
    ;
    
    // clear canvas_buf
    for i in 0..self.canvas_buf.len() {
      self.canvas_buf[i] = " ";
    }
        
    // set canvas_buf for each entity type, in order
    for typ_k in typ_k_vec {
      self.set_canvas_buf_typ(typ_k);
    }
  }
  
  fn tick_typ(&mut self, player_input: &PlayerInput) {
    
    // loop through entities_of_typ and do stuff
    for entity in self.entities.iter_mut() {
      if entity.typ_k == "player" {

        if player_input.left {
          entity.x -= 1;
        }

        if player_input.right {
          entity.x += 1;
        }

      }
    }
  }
  
  fn set_canvas_buf_typ(&mut self, typ_k: &str) {

    // filter by provided type
    let entities = self.entities
      .iter_mut()
      .filter(|entity| entity.typ_k == typ_k)
      .collect::<Vec<_>>()
    ;
    
    // replace canvas_buf row position with entity utf_art
    for entity in entities {
      self.canvas_buf[entity.x] = entity.utf_art;
    }
  }
}

impl Iterator for World<'_> {
  type Item = f32;

  fn next(&mut self) -> Option<Self::Item> {
    self.tick();
    self.t += 1.;
    Some(self.t)
  }
}

fn main() {
  
  // define the player input
  let mut player_input = PlayerInput {
    up: false,
    down: false,
    right: false,
    left: false,
  };

  // entity type paint order
  let entity_typ_order = vec!["thing", "player"];
  
  // game loop
  let _dt = 1.;
  let mut world = World::new(entity_typ_order);
  
  let mut cmd;
  
  loop {
    // iterate world until it it ends
    if world.next() == None {
      break;
    }

    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    // print canvas_buf to terminal
    for elem in &world.canvas_buf {
      print!("{}", elem);
    }
    println!("");

    // read user input
    println!("enter `a` to move left or `d` to move right: ");
    cmd = "".to_string();
    io::stdin()
    .read_line(&mut cmd)
    .expect("Failed to read line");
    
    // reset player input
    player_input.left = false;
    player_input.right = false;
    
    // match possible commands
    match cmd.as_str().trim() {
      "a" => player_input.left = true,
      "d" => player_input.right = true,
      string => println!("unrecognized command: {}", string)
    }
    
    // send input and process entities
    world.tick_typ(&player_input);

  }
  println!("The End.");
}

