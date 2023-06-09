#!/usr/bin/env nu

def main [
  id: int
  name: string
] {
  cargo test --example ('Item' + ($id | into string) ) $name -- --nocapture
}
