```
$ cargo component build                                                         
   Compiling print-preopened v0.1.0 (/Users/rajatjindal/go/src/github.com/rajatjindal/print-preopened)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
    Creating component target/wasm32-wasi/debug/print_preopened.wasm


$ wasmtime run --dir src --dir wit target/wasm32-wasi/debug/print_preopened.wasm
src
wit

$ spin up                                                                       
/
```