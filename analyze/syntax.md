# AetherScript

## Vision
- AltJS (source-to-source compiler targeting JavaScript) that empowers you to write JavaScript more safely and robustly
- Rust-like syntax (no borrow checker)
- Track side effect for optimize

## Code
```
// inline comment
/*
  multi line comment
*/
fn func(counter: mut Box<smi>, mut insts: [Inst]) {
    // counter = Box::new(0); // variable mutation, it's ng
    counter.inner += 1; // reference mutation, it's ok

    insts = insts[(counter.inner)..]; // it's ok
    // insts.push(Inst::End) // it's ng

    let inst: Inst = insts[0];
    if inst == Inst::End {
        return;
    }

    let mut x = 0;
    let incr_x = || {
        x += 1;
        x
    };
}

async fn get_example(): str {
    match await fetch("https://example.com") {
        AsyncResult::Fullfield(response) => (await response.text()).unwrap(),
        AsyncResult::Rejected(err) => panic!(err),
    }
}

struct Box<T> {
    inner: T
}
impl<T> Box<T> {
    pub fn new(val: T): Box {
        Box { inner: val }
    }
}

enum Option<T> {
    None,
    Some(T),
}
impl<T: Clone> Clone for Option<T> {
    fn clone(self): Option<T> {
        match self {
            Option::Some(val) => Option::Some(val.clone()),
            Option::None => Option::None,
        } 
    }
}
```

## Types

### Primitive
Can't create direct mutable reference, requires Box
- num
- smi
- str
- bool
- func
- unique (symbol)

### Object
Can create direct mutable reference
- \[T\] (array)
- instance
  - struct
    - Box
  - enum
    - Option
    - Result
- tuple

## Function attributes
```
#[inline(always)]
#[inline(auto)]
#[inline(never)]

#[unsafe(pure)]
#[impure]
```
