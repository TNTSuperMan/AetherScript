# AetherScriptアイデア

AltJS

## Concepts
```
struct Counter {
    count: smi,
}
impl Counter {
    new(init: Option<smi>): Counter {
        Counter { counter: init.or(0) }
    }
    increment(mut self) {
        self.count += 1;
    }
}

const counter = Counter::new(None);

counter.increment();

print(counter.count);

```

```js
const Counter__construct = (init) => {
    return [init ?? 0];
}
const Counter_increment = (_this) => {
    _this[0 /* count */]++;
}

const counter = Counter__construct();

Counter_increment(counter);

print(counter)

```

before `class D{count;constructor(e){this.count=e??0}increment(){this.count++}}var f=new D;f.increment();print(f.count);`
after  `var c=e=>[e??0],i=e=>{e[0]++},f=c(null);i(f);print(f[0]);`

## 型
### プリミティブ
値の同一性を持っての可変操作が不可能なもの
- bigint
- smi
- number
- string
- bool
- symbol

### Noプリミティブ
値の同一性を保って可変操作が可能なもの
- function キャプチャしてる変数が可変になる
- object   JavaScriptのオブジェクトと一致するフラット構造
- instance 中身はtuple コンストラクタじゃなくてRustみたいに生成
- tuple
- array

```
impl @array {
    pub push()
}
```

## 構文アイデア

### unsafe block
```
fn print(msg: string) {
    unsafe {
        @Global.console.log(msg);
    }
}
```

### Promise match
```
match await promise_val {
    Fullfield(val) => {}
    Rejected(err) => {}
}
```

* await promiseの値は`enum PromiseResult<T, E> { Fullfield(T), Rejected(E) }` ってことにする。インライン展開とかでいい感じになることを期待★

### Side effect attribute
コンパイラ内部で出るやつ
最適化や展開に使う
```
fn print_smi(a: smi) { // impure
    print(a.toString());
}
fn add_smi(a: smi, b: smi): smi { // pure
    a + b
}

#[unsafe(impure)]
fn print(msg: string) {
    unsafe {
        @Global.console.log(msg);
    }
}
#[unsafe(pure)]
fn rand(): num {
    unsafe {
        @Global.Math.random()
    }
}
```

### Immutable variable
```
fn print_nums(nums: [smi]) { // immutable
    for (let num in nums) {
        print(num.toString());
    }
}
fn zerofill_nums(nums: mut [smi]) { // mutable
    for (let i in range(0, nums.len())) {
        nums[i] = 0;
    }
}
```

