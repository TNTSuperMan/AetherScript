declare namespace AST {
    type Type =
        | { type: "bigint" }
        | { type: "integer" }
        | { type: "float" }
        | { type: "string" }
        | { type: "bool" }
        | { type: "symbol" }
        | { type: "function", args: Type[], result: Type }
        | { type: "instance", id: symbol }
        | { type: "object",  }
        | { type: "array", element: Type }
        | { type: "tuple", elements: Type[] }
    declare namespace Expressions {
        type Identifier = { type: "identifier", name: string }
        type Number = { type: "number", numeral: 2 | 8 | 10 | 16, big: boolean, value: string }
        type String = { type: "string", value: string }
        type Bool = { type: "bool", value: boolean }
        type Array = { type: "array", elements: Expression[] }
        type Call = { type: "call", function: Expression, args: Expression[] }
        type Member = { type: "member", from: Expression, key: string }
        type Binary = { type: "binary", operation: "+" | "-" | "*" | "/" | "%" | "**" | "|" | "&" | "^" | "&&" | "||" | "^^", left: Expression, right: Expression }
        type Unary = { type: "unary", operation: "+" | "-" | "~" | "!" }
        type Function = { type: "func", args: Identifier[], blocks: never[], argsType: Type[], resultType: Type }
    }
    type Expression =
        | Expressions.Identifier
        | Expressions.Number
        | Expressions.String
        | Expressions.Bool
        | Expressions.Array
        | Expressions.Call
        | Expressions.Member
        | Expressions.Binary
        | Expressions.Unary
        | Expressions.Function
    declare namespace Statements {
        type Call = { type: "call", function: Expression, args: Expression[] }
        type Assign = { type: "assign", target: Expressions.Identifier | Expressions.Member, value: Expression }
        type VariableDeclaration = { type: "vardeclare", name: string, init: Expression, type?: Type }
        type If = { type: "if", condition: Expression }
    }
    type Statement =
        | Statements.Call
        | Statements.Assign
        | Statements.VariableDeclaration
        | Statements.If
}
