fn main() {
    #[cfg(not(any(feature = "hashbrown", feature = "hashmap", feature = "btreemap",)))]
    compile_error! {
        "One of `hashbrown`, `hashmap` or `btreemap` feature should be enabled"
    }
}
