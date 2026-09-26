# my lang concepts

## コンセプト(日本語)
- より安全で堅牢なJavaScript
- Rustライク文法(borrow checkerまでは無い)
- 副作用等を追跡することで、より高度な最適化・インライン化を行う

## code
```
// inline comment
/*
  multi line comment
*/
fn func(mut index: smi, counter: mut Box<smi>, mut insts: [Inst]) { // mut var, mut ref, mut var
    index += 1; // local mutate
    counter.inner += 1; // global mutate
    
    // insts.push(""); // immutable on global, error
    insts = insts[index..]; // local mutate, ok

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
    let response = match await fetch("https://example.com") {
        AsyncResult::Fullfield(response) => response,
        AsyncResult::Rejected(err) => throw err,
    };
    (await response.text()).unwrap()
}

struct Box<T> {
    inner: <T>
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

## types

### primitive
can't create mutable reference, need Box
- num
- smi
- str
- bool
- func
- unique (symbol)

### imprimitive
can create mutatble reference
- \[T\] (array)
- instance
  - struct
    - Box
  - enum
    - Option
    - Result
- tuple

## function attributes
```
#[inline(always)]
#[inline(auto)]
#[inline(never)]

#[unsafe(pure)]
#[impure]
```
