// This module contains tests for the `compose!` macro to ensure it works as expected.

#[cfg(test)]
mod tests {
    use compose_macro::compose;

    // A simple function that multiplies its input by 2.
    fn h(x: i32) -> i32 {
        x * 2
    }

    // A simple function that multiplies its input by 3.
    fn g(x: i32) -> i32 {
        x * 3
    }

    // A simple function that multiplies its input by 4.
    fn k(x: i32) -> i32 {
        x * 4
    }

    // Test the `compose!` macro by chaining `h`, `g`, and `k`.
    #[test]
    fn it_works() {
        // Compose the functions: h -> g -> k.
        let composed_function = compose!(h -> g -> k);

        // Apply the composed function to the input `2`.
        let result = composed_function(2);

        // Debug print the result for verification.
        dbg!(result); // Expected output: 48 (2 * 2 * 3 * 4)
    }
}