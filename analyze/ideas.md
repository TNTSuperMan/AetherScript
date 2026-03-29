# AetherScript アイデア

AltJS

## サンプル

### 1
```ts
class Counter {
    count: integer;
    static construct(init?: integer) {
        return Counter { count: init ?? 0 };
    }
    increment() {
        this.count++;
    }
}

const counter = new Counter();

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
after  `var c=e=>[e??0],i=e=>{e[0]++},f=c();i(f);print(f[0]);`

### 2
```ts
class ShortyBinary {
    binary: Uint8Array;
    static construct(): ShortyBinary {
        return ShortyBinary { binary: new Uint8Array() };
    }
    static from(binary: Uint8Array): ShortyBinary {
        return ShortyBinary { binary: binary };
    }
    get(index: integer): integer {
        return this.binary[index];
    }
    set(index: integer, value: integer) {
        this.binary[index] = value;
    }
}

const bin = ShortyBinary.from(b);

bin.set(0, bin.get(0) | 0b10000000);

```

```js
const ShortyBinary__construct = () => {
    return [new Uint8Array()  /* binary */];
}
const ShortyBinary__from = (binary) => {
    return [binary  /* binary */];
}
const ShortyBinary_get = (_this, index) => {
    return _this[0  /* binary */][index];
}
const ShortyBinary_set = (_this, index, value) => {
    _this[0 /* binary */][index] = value;
}

const bin = ShortyBinary__from(b);

ShortyBinary_set(bin, 0, ShortyBinary_get(bin, 0) | 0b10000000);

```

before `class S{binary;constructor(){this.binary=new Uint8Array()}static from(e){var t=new S;t.binary=e;return t}get(e){return this.binary[e]}set(e,t){this.binary[e]=t}}var bin=new S;bin.set(0,bin.get(0)|128);`
after  `var f=n=>[n],g=(t,i)=>t[0][i],s=(t,i,v)=>{t[0][i]=v},e=f(b);s(e,0,g(e,0)|128);`

## 型
### プリミティブ
- bigint
- integer
- float
- string
- bool
- symbol

### Noプリミティブ
- function Rustみたいにself指定みたいなのも入れる
- object   JavaScriptのオブジェクトと一致するフラット構造
- instance 中身はtuple コンストラクタじゃなくてRustみたいに生成
- tuple
- array


```
impl @array {
    pub push()
}
```
