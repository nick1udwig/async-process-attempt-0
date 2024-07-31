# Async Kinode process attempt 0

## Comments on usage

### wit-bindgen

~/git/wit-bindgen from cpetig :
https://github.com/dicej/wit-bindgen/pull/2
with the following diff:
```
--- a/crates/rust/src/bindgen.rs
+++ b/crates/rust/src/bindgen.rs
@@ -936,6 +936,11 @@ impl Bindgen for FunctionBindgen<'_, '_> {
                     })
                     .collect::<Vec<_>>()
                     .join(", ");
+                let params = if params.is_empty() {
+                    "()".to_string()
+                } else {
+                    params
+                };
                 let async_support = self.gen.path_to_async_support();
                 uwriteln!(
                     self.src,
```

### wasm-tools

Install this branch from cpetig:
https://github.com/dicej/wasm-tools/pull/1

## Status

Didn't quite get it working.
Did get this compiling, but couldn't get runtime compiling before I gave up.

Based on this work:
https://github.com/dicej/component-async-demo

Note Kinode wit had to be modified to export an interface rather than a function.

## History of thought

https://gist.github.com/nick1udwig/60c689bbc084f5aa655b16ae8b1c211d
https://discord.com/channels/1186394868336038080/1192475661756018849/1268212177265754163

## Other useful links:

https://github.com/dicej/wasm-tools/pull/1
https://github.com/dicej/wit-bindgen/pull/2
https://bytecodealliance.zulipchat.com/#narrow/stream/223391-wasm/topic/Component.20Model.20Implementation.20Bi-Weekly/near/439301362
https://github.com/bytecodealliance/wasmtime/blob/87817f38a128caa76eaa6a3c3c8ceac81a329a3e/crates/wasi/src/stream.rs
https://github.com/bytecodealliance/wasmtime/blob/87817f38a128caa76eaa6a3c3c8ceac81a329a3e/crates/wasi/src/poll.rs#L14
https://github.com/dicej/isyswasfa
