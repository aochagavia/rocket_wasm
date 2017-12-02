from subprocess import call

call(['wasm-gc', 'target/wasm32-unknown-unknown/release/rocket.wasm', 'static/rocket.wasm'])
