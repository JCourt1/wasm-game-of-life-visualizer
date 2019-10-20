<div align="center">

  <h1><code>wasm-game-of-life-visualizer</code></h1>
  
</div>

## About

A visualizer for Conway's game of life built using Rust compiled to WebAssembly.

This project uses <a href="https://github.com/rustwasm/wasm-pack-template">wasm-pack-template</a>.

## 🚴 Usage

The web assembly can be built using:

```
wasm-pack build
```

Then, install Javascript dependencies and run local server with:

```
cd www
npm install && npm run start
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
