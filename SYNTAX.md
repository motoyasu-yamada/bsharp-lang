B# Syntax Specification
===
https://users-cs.au.dk/amoeller/RegAut/JavaBNF.html

## Syntax
- `Program`                   ::= `Statements` | `FunctionDeclaration` "EOF"
- `FunctionDeclaration`       ::= `Fn` "IDENT" "EOL" 
                                  `Statements`
                                  `End`
- `Statements`                ::= `Statement` | `Statements` "EOL" `Statement` 
- `Statement`                 ::= `EmptyStatement` | 
                                  `IfBlock` | 
                                  `ForBlock` |
                                  `Declaration` |
                                  `ExpressionStatement`
- `EmptyStatement`            ::= ;
- `ForBlock`                  ::= "For" `ForCounter` "=" `Expression` "To" `Expression` "EOL"
                                    `Statements`
                                  "Next"
- `IfBlock`                   ::= "If" `Expression` "Then" "EOL"
                                    `Statements`
                                  "End" "If"
- `Declaration`               ::= "Const" `Assignment` |
                                  "Dim" `Assignment`
- `ExpressionStatement`       ::= `Assignment` | 
                                  `MethodInvocation`                                  
- `Assignment`                ::= `LeftHandSide` "="  `Expression`
- `LeftHandSide`              ::= `VariableAccess` | `ArrayAccess`
- `Expression`                ::= `LogicalXorExpression`
- `LogicalXorExpression`      ::= `LogicalOrExpression` | 
                                  `LogicalXorExpression` "Xor" `LogicalOrExpression`
- `LogicalOrExpression`       ::= `LogicalAndExpression` | 
                                  `LogicalOrExpression` "Or" `LogicalAndExpression`
- `LogicalAndExpression`      ::= `LogicalNotExpression` | 
                                  `LogicalAndExpression` "And" `LogicalNotExpression`
- `LogicalNotExpression`      ::= "Not" `LogicalNotExpression` | 
                                  `EqualityExpression`
- `EqualityExpression`        ::= `AdditiveExpression` | 
                                  `EqualityExpression` "=" `AdditiveExpression` | 
                                  `EqualityExpression` "<>" `AdditiveExpression` | 
                                  `EqualityExpression` "<" `AdditiveExpression` | 
                                  `EqualityExpression` ">" `AdditiveExpression` | 
                                  `EqualityExpression` "<=" `AdditiveExpression` | 
                                  `EqualityExpression` ">=" `AdditiveExpression`
- `AdditiveExpression`        ::= `MultiplicativeExpression` | 
                                  `AdditiveExpression` "+" `MultiplicativeExpression` | 
                                  `AdditiveExpression` "-" `MultiplicativeExpression`
- `MultiplicativeExpression`  ::= `UnaryExpression` | 
                                  `MultiplicativeExpression` "*" `UnaryExpression` | 
                                  `MultiplicativeExpression` "/" `UnaryExpression` | 
                                  `MultiplicativeExpression` "%" `UnaryExpression`
- `UnaryExpression`           ::= "+" `UnaryExpression` | 
                                  "-" `UnaryExpression` | 
                                  `Exponential`
- `ExponentialExpression`     ::= `Primary` | 
                                  `Primary` "^" `ExponentialExpression`
- `Primary`                   ::= `Literal` | `VariableAccess` | Me | ( `Expression` )  | `MethodInvocation` | `ArrayAccess`
- `MethodInvocation`          ::= `MethodName` ( ``ArgumentList``? )
- `ArgumentList`              ::= `Expression` | 
                                  `ArgumentList` , `Expression`
- `ArrayAccess`               ::= `GetVariable` [ `Expression` ] | 
                                  `Primary` [ `Expression`]
- `VariableAccess`            ::= `Identifier`
- `MethodName`                ::= `Identifier`


