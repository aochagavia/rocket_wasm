from shutil import copyfile
from subprocess import call

copyfile('target/wasm32-unknown-unknown/release/rocket.wasm', 'html/rocket.wasm')
call(['wasm-gc', 'html/rocket.wasm', 'html/program.wasm'])
