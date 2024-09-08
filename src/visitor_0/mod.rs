mod item;
mod visitor;
mod visitor_acceptor;
mod half_value_visitor;

use self::half_value_visitor::HalfValueVisitor;
use self::item::Item;
use self::visitor_acceptor::VisitorAcceptor;

const ITEM_0: Item = Item { value: 0 };
const ITEM_1: Item = Item { value: 1 };
const ITEM_2: Item = Item { value: 2 };

pub fn example() {
  let mut items: Vec<Item> = vec![ITEM_0, ITEM_1, ITEM_2];

  println!("{items:?}");

  items.iter_mut().for_each(|item| item.double_value());

  println!("{items:?}");

  items.iter_mut().for_each(|item| item.accept_visitor(&HalfValueVisitor));

  println!("{items:?}");
}
