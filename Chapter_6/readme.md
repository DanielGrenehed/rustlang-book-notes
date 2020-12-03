# Enums and Pattern Matching

## Defining an Enum
Enums allow storage of different kinds of data under one name. Enums can have functions, like structs. Use the 'match' expression for enum control flow. Useful standard library enum is Option<t> which contains either a value t(Some(T)) or no value(None).

## The match Control Flow Operator
The compiler confirms all possible cases are handled in a match expression. 'match' expressions can return values of any type. We can use the '_' placeholder to match each case that is not explicitly handled.

## Concise Control Flow with if let
We can use 'if let Some(x) = some_value {}' to cover just a one match.
