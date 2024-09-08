mod item;

use item::Item;

const ITEM_0: Item = Item { health: 0, wealth: 0., wisdom: 0 };
const ITEM_1: Item = Item { health: 1, wealth: 1., wisdom: 1 };
const ITEM_2: Item = Item { health: 2, wealth: 2., wisdom: 2 };

pub fn example() {
  let items: Vec<Item> = vec![ITEM_0, ITEM_1, ITEM_2];

  items.iter().for_each(|item| println!("item: {item:?}"));
}
