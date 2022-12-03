use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?.parse()?;
    
    let items = input.lines();

    let mut max_calories: i32 = 0;
    let mut current_inventory: i32 = 0;

    for item in items {
      if item != "" {
        current_inventory += item.parse::<i32>().unwrap();
      } else {
        if max_calories < current_inventory {
          max_calories = current_inventory;
        }
        current_inventory = 0;
      }
    }

    println!("{max_calories}");
    Ok(())
}