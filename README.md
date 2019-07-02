# trace

**Setup**

curl https://sh.rustup.rs -sSf | sh

rustup toolchain install nightly

rustup target add wasm32-unknown-unknown --toolchain nightly

cargo +nightly install wasm-bindgen-cli

cargo install https

**Building**

cargo +nightly build --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/debug/trace.wasm --no-modules --no-modules-global rust --out-dir build

cargo +nightly build --release --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/release/trace.wasm --no-modules --no-modules-global rust --out-dir build

**Testing**

http