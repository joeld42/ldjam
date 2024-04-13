cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "basecodewasm" .\target\wasm32-unknown-unknown\release\basecode_bevy_rs.wasm