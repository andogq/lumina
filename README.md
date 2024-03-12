# Language

```rs
// Variable declarations
let x = 1;
let y = 2;

// Operations
let result = x + y;

// Blocks
{
    let b = 1;
}

// Blocks as expressions
let c = {
    3
};

// If statements as expressions
let a = if result % 2 == 0 {
    123
} else {
    999
};

// Function parameters and return types must be typed
fn my_func(a: usize, b: usize) -> usize {
    // If statements
    if a == b {
        // Explicit returns
        return 0;
    }

    // Implicit returns with no semicolon
    a + b
}

// Function calls
my_func(x, y);
```
