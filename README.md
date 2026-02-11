# Musath

### Music Makes Math

Musath is a mathematical composition software. It takes a function `output(t)` as an entry point, and renders the function as audio.

## Usage

There are 2 ways to use Musath: as an interpreter and as a library.

### Interpreter

The Musath binary can take in a `.mth` file and render it to a `.wav` file.

`example.mth`

```
TITLE = "example"
DURATION = 10

output(t) = sin(440 * t * tau)
```

In this example, `output(t)` produces a 440Hz sine wave. To render this to audio, run the following:

`musath example.mth`

This will produce `example.wav` in the current working directory.

`.mth` files consist of a header and a body. The header section is a simple collection of key-value pairs that describe metadata about the composition.

| Key | Meaning |
|-|-|
| TITLE | The output filename |
| DURATION | The length of the composition in seconds |

The body of a `.mth` file is a collection of function declarations. One of the functions *must* have the signature `output(t)`, and this will be the entry point.

The functions definition syntax is bespoke to Musath, but should be consistent with most convetions of writing math in plaintext.

```
some_function(a, b, c, d) = floor(a) * (b + c) ^ d
```

| Operator | Meaning |
|-|-|
| + | Addition |
| - | Subtraction/Negation |
| * | Multiplication |
| / | Division |
| % | Euclidian Remainder |
| ^ | Exponentiation |

There are also a few built-in values:
| Identifier | Meaning |
|-|-|
| e | Euler's Constant |
| pi | The ratio of a circles circumference to its diameter |
| tau | 2 * pi |

As well as a some built-in functions:

| Signature | Meaning | Example | Value |
|-|-|-|-|
| `abs(x)` | Absolute value of `x` | `abs(-1)` | `1` |
| `min(l, r)` | Minimum value between `l` and `r` | `min(2,3)` | `3` |
| `max(l, r)` | Maximum value between `l` and `r` | `max(4,5)` | `5` |
| `floor(x)` | Floor of `x` | `floor(0.5)` | `0` |
| `ceil(x)` | Ceiling of `x` | `ceil(0.5)` | `1` |
| `sin(x)` | Sine of `x` radians | `sin(pi/2)` | `1` |
| `cos(x)` | Cosine of `x` radians | `cos(0)` | `1` |
| `sum(x, start, end, expression)` | The sum of the evaluations of `expression` substituting `x` with every integer from `start` (inclusive) to `end` (exclusive) | `sum(n,1,5,n*2)` | `20` |
| `proc(x)` | The product of the evaluations of `expression` substituting `x` with every integer from `start` (inclusive) to `end` (exclusive) | `prod(n,1,5,n+1)` | `120` |

### Library

The interpreter is a bit slow, and not as extensible as a full language like Rust. It is also possible to write a Rust binary that produces audio using Musath as a library.

```rust
use std::f64::consts::TAU;

use musath::{
    composition::Composition,
    renderer::{Renderer, parallel_renderer::ParallelRenderer},
};

fn main() {
    ParallelRenderer::default()
        .render(&Composition::from_function("beat", 10.0, output))
        .unwrap();
}

fn decay(t: f64, period: f64, tension: f64, phase_shift: f64) -> f64 {
    (1.0 - ((t - phase_shift).rem_euclid(period)) / period).powf(tension)
}

fn noise(t: f64) -> f64 {
    (2.0f64.powf(t + 100.0)).sin().powf(2.0)
}

fn step(t: f64, freq: f64) -> f64 {
    ((t * freq * 2.0).rem_euclid(2.0)).floor()
}

fn voice(t: f64) -> f64 {
    step(t, 440.0) / 10.0 * (1..4).map(|n| -step(t, n as f64) + 1.0).product::<f64>()
}

fn square(t: f64) -> f64 {
    (1..5).map(|n| voice(t - (n as f64 * 0.25 - 0.25)) / 4.0f64.powf(n as f64 - 1.0)).sum()
}

fn output(t: f64) -> f64 {
    noise(t) * decay(t, 1.0, 8.0, 0.5) +
    (110.0 * t * TAU).sin() * decay(t, 1.0, 8.0, 0.0) +
    square(t)
}
```

This method is much faster to render and more extensible (possibly too extensible if one wants to confine their compositions to those which can be written as closed-form mathematical functions).
