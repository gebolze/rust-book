fn main() {
    // by default panic! does unwind the stack, as this can be a lot of work.
    // To prevent stack unwinding, you could modify the cargo.toml to abort
    // instead, which won't cleanup the memory. (see cargo.toml)
    panic!("crash and burn");
}
