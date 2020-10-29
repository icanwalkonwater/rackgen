# Rackgen

## Cli usage
```
cargo run -- --help
cargo run -- test.so
cargo run -- test.so --class CustomName
```

## Lib usage
```
cargo build --release --lib
```

```c
char* js = gen_js_interface_for_rack("./test.so", NULL);
// do something with the text
free_gen_js(js);
```