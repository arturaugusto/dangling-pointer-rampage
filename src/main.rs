use std::io;
use crate::things::{World, PlayerInput};

mod things;


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
  let mut world = World::new(10, 6, entity_typ_order);
  
  let mut cmd;
  
  loop {
    // iterate world until it it ends
    if world.next() == None {
      break;
    }

    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    // print canvas_buf to terminal
    for (i, elem) in world.canvas_buf.iter().enumerate() {
      if (i+1) % world.w == 0 {
        println!("{}", elem);
      } else {
        print!("{}", elem);
      }
    }
    println!("");

    // read user input
    println!("enter `a` to move left or `d` to move right: ");
    cmd = "".to_string();
    
    io::stdin()
      .read_line(&mut cmd)
      .expect("Failed to read line")
    ;
    
    // reset player input
    player_input.left = false;
    player_input.right = false;
    player_input.down = false;
    player_input.up = false;

    
    // match possible commands
    match cmd.as_str().trim() {
      "a" => player_input.left = true,
      "d" => player_input.right = true,
      "s" => player_input.down = true,
      "w" => player_input.up = true,
      string => println!("unrecognized command: {}", string)
    }
    
    // send input and process entities
    world.tick_typ(&player_input);

  }
  println!("The End.");
}

