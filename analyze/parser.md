## Lexer
- `[Id]`
- `[Literal]`
  - `[Literal:String]`
  - `[Literal:Int]`
  - etc
- wordsymbol, symbol: e.g. `async fn () {}`
- ast module: `{Expr}`

## Lexer to AST rule

### notes

* `{Any}, {Any}, {Any}, }`の並びが多いので共通化する？ `parse_arrayish<ELEMENT: ASTModule>(token_stream: _, finish: Lexer)`
* ASTモジュールはtraitにしとく？ `trait ASTModule { fn try_parse(token_stream: _): Result<Self, _> }`

### module statement
1. match
    - `async` -> expect `fn {Func}`
    - `fn` -> expect `{Func}`
    - `struct` -> expect `{Struct}`
    - `enum` -> expect `{Enum}`

### Func (without prefix) `add_smi(mut a: smi, b: smi): { a += b; }`
1. expect `[Id] (`
2. match
    - `)` -> jump 3
    - _ -> expect `{Arg}`, switch
        - `,` -> jump 2
        - `)` -> jump 3
3. match
    - `{` -> go 4*
    - `:` -> expect `{Type} {`
    - `<` -> expect `{Generics}`
4. match
    - `}` -> finish
    - _ -> expect `{FuncStmt}`, jump 4

### Struct (without prefix)
1. expect `[Id]`
2. match
    - `{` -> go 3*
    - `<` -> expect `{Generics}`
3. match
    - `}` -> finish
    - _ -> expect `[Id]: {Type}`, match
        - `,` -> jump 3
        - `}` -> finish

### Enum (without prefix)
1. expect `[Id]`
2. match
    - `{` -> go 3*
    - `<` -> expect `{Generics}`
3. match
    - `}` -> finish
    - _ => expect `[Id]`, match
        - `}` -> finish
        - `,` -> jump 3
        - `(` -> **todo**
        - `{` -> **todo**

### Generics (without prefix) `T, E: Error>`
1. match
    - `{Generic}` -> match
        - `>` -> finish
        - `,` -> jump 1
    - `>` -> finish
