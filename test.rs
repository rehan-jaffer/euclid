struct a {
  something: something
}

struct b<'z> {
  sa: &a
}

fn main() {
  let new_b = b { a: &a { } };
}
