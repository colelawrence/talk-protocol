
1.
```rust
let st1 = state![(you) points ("up") at /target/];
let st2 = state![/target/ has bbox /bbox/];
```
2.
```rust
let q1 = query![
    (you) points ("up") at /target/,
    /target/ has bbox /bbox/
];
```
3.
```rust
system! {
    When (you) points ("up") at /target/ {

    }
}
```