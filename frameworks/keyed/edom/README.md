Because of the heavy amount of Rust dependencies, this example is pre-compiled, so you don't need to compile anything.

However, if you do want to compile it, you will need the following:

* [Rust](https://www.rust-lang.org/tools/install)

* [wasm-pack](https://rustwasm.github.io/wasm-pack/)

After installing those, run these commands:

```
npm install
export NODE_OPTIONS=--openssl-legacy-provider  # :( npm audit --force didn't fix the issue
npm run build-prod-force
```
