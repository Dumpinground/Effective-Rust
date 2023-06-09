#!/usr/bin/env nu

# test a example
def main [
  id: int # Input example item id. Test in Item1 when id = 1.
  name = '' # Input test function name. Default to show all tests.
] {
  cargo test --example ('item' + ($id | into string) ) $name -- --nocapture
}
