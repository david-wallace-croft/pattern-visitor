use self::original::example_original;
use self::supplemented::example_supplemented;

mod original;
mod supplemented;

pub fn example() {
  example_original();

  example_supplemented();
}
