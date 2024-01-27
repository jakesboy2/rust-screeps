## Derive(Clone)
* #[derive(Clone)] on the `enum CreepTarget` code implements the copy trait for everything inside of it (if possible)

## Storing memory on the heap
```rust
thread_local! {
    static CREEP_TARGETS: RefCell<HashMap<String, CreepTarget>> = RefCell::new(HashMap::new());
}
```