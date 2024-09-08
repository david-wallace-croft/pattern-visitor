mod item;
mod visitor;
mod visitor_acceptor;
mod half_value_visitor;

use self::half_value_visitor::HalfValueVisitor;
use self::item::Item;

const ITEM_0: Item<usize> = Item { value: 0 };
const ITEM_1: Item<usize> = Item { value: 1 };
const ITEM_2: Item<usize> = Item { value: 2 };

pub fn example() {
  let mut items: Vec<Item<usize>> = vec![ITEM_0, ITEM_1, ITEM_2];

  println!("{items:?}");

  items.iter_mut().for_each(|item| item.double_value());

  println!("{items:?}");

  let half_value_visitor = HalfValueVisitor::<usize>::default();

  items.iter_mut().for_each(|item| item.accept_visitor(&half_value_visitor));

  println!("{items:?}");
}
