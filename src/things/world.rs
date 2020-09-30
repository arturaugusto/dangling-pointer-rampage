#[derive(Debug)]
pub struct PlayerInput {
  pub up: bool,
  pub down: bool,
  pub right: bool,
  pub left: bool,
}


#[derive(Debug)]
pub struct Entity<'a> {
  id: usize,
  x: usize,
  y: usize,
  typ_k: &'a str,
  del: bool,
  utf_art: Vec<&'a str>,
}


pub struct World<'a> {
  pub w: usize,
  pub h: usize,
  t: f32,
  entities: Vec<Entity<'a>>,
  entity_typ_order: Vec<&'a str>,
  pub canvas_buf: Vec<&'a str>,
}

impl World<'_> {
  pub fn new(w: usize, h: usize, entity_typ_order: Vec<&str>) -> World {

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
        utf_art: if i % 2 == 0 {vec!["#", "#"]} else {vec!["@", "@"]},
        del: true,
      };
      entities.push(entity)
    }

    // initialize canvas buffer  
    let canvas_w = w * h;
    let mut canvas_buf: Vec<&str> = Vec::with_capacity(canvas_w);
    for _i in 0..canvas_w {
      canvas_buf.push(" ");
    }

    World {
      w: w,
      h: h,
      t: 0.0,
      entities: entities,
      entity_typ_order: entity_typ_order,
      canvas_buf: canvas_buf,
    }
  }
  
  pub fn tick(&mut self) {
        
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
  
  pub fn tick_typ(&mut self, player_input: &PlayerInput) {
    
    // loop through entities_of_typ and do stuff
    for entity in self.entities.iter_mut() {
      if entity.typ_k == "player" {

        if player_input.left && entity.x > 0 {
          entity.x -= 1;
        }

        if player_input.right && entity.x < self.w - entity.utf_art.len() {
          entity.x += 1;
        }

        if player_input.down && entity.y < self.h - 1 {
          entity.y += 1;
        }

        if player_input.up && entity.y > 0 {
          entity.y -= 1;
        }


      }
    }
  }
  
  pub fn set_canvas_buf_typ(&mut self, typ_k: &str) {

    // filter by provided type
    let entities = self.entities
      .iter_mut()
      .filter(|entity| entity.typ_k == typ_k)
      .collect::<Vec<_>>()
    ;
    
    // replace canvas_buf row position with entity utf_art
    for entity in entities {
      for (i, c) in entity.utf_art.iter().enumerate() {
        //println!("{}", entity.x + entity.y*self.w);
        self.canvas_buf[i + entity.x + entity.y*self.w] = c;
      }
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
