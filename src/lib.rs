
#[cfg(test)]
mod tests {
    use compose_macro::compose;

    fn h(x: i32) -> i32 {
      x * 2
    }
    fn g(x: i32) -> i32 {
      x * 3
    }
    fn k(x: i32) -> i32 {
      x * 4
    }

    #[test]
    fn it_works() {
        dbg!(compose!(h -> g -> k)(2));
    }
}
