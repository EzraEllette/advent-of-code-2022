use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?.parse()?;
    
    let items = input.lines();

    let mut inventories: Vec<i32> = Vec::new();
    let mut current_inventory: i32 = 0;

    for item in items {
      if item != "" {
        current_inventory += item.parse::<i32>().unwrap();
      } else {
        inventories.push(current_inventory);
        current_inventory = 0;
      }
    }

    inventories.sort_unstable();

    let start_index = inventories.len();
    let mut sum: i32 = 0;

    for num in start_index-3..start_index {
      sum += inventories[num];

    }

    println!("{sum}");
    Ok(())
}