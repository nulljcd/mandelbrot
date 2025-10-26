# mandelbrot
A simple mandelbrot fractal renderer in rust.

```
cargo build --release
./target/release/mandelbrot [real start position] [imaginary start position] [scale]
```
The start position defines the top left of the render.
```
./target/release/mandelbrot -0.95 0.33 0.1
```
<img width="4096" height="4096" alt="output-1" src="https://github.com/user-attachments/assets/3eb3787c-a1f5-4356-9e55-431e81f5f203" />
