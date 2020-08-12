# 3 Common Programming Concepts

## 1 Variables and Mutability
Immutable variables cannot be changed once they have been set.

Rust is immutable by default so it is easier to track down bugs since the values once set won't change.

Indicating a variable to be mutable will make it easier to see what to expect from a variable.

Constants are never changing, the "mut" keyword won't change that, they are always immutable as compared to immutable variables in Rust, the variable will be mutable when setting it with "mut"

Constants are created with the "const" keyword in Rust and the value has to be set on the same line, and have to be computable at compile-time.

Constants are valid in the declared scope the entire program runtime.

Shadowing is replacing the value of a variable, you can alter a immutable variable by setting it again with the "let" keyword.

Shadowing also allows one to store another type with the variable name, which is not permitted when declaring a variable mutable.

## 2 Data Types
Specify the new type when shadowing a variable when multiple types are possible.

The Scalar types represents a single value. The primary scalar types in Rust are Integers, floating-point numbers, Booleans and characters.

The Integer type in Rust: 8, 16, 32, 64, 128 - bit and "size", with i or u preceding the length indicating signed or unsigned integer, ie. u16 will be a 16-bit unsigned integer capable of storing numbers from 0 to 2^16 - 1. The "size" uses bit-size depending on the os architecture, 64-bit or 32-bit. i32 is Rust's default integer type.

You can write integer literals with decimal(90_321), hex(0xff), octal(0o73), binary(0b1010_1001) and byte(b'A , only u8 ints).

Floating-point type in Rust have the possibility of being either 64-bit(f64) or 32-bit(f32). The f32 is a single-precision float and f64 has double recision.

The Boolean type is capable of being either "true" or "false", the type is specified using "bool" and is one byte in size.

The Character type is specified with "char" and is Rust's most primitive alphabetic type. char literals are specified with single quotes, as opposed to string literals, using double quotes. Character types are 4-byte sized and represented with Unicode Scalar Value.

Compound Types can store multiple values in one type, Rust's two primitive compound types are tuples and arrays.

Tuples have a fixed size but can store multiple values of different types. The types stored are enclosed in parentheses separated with comma. You can access the individual values with either creating a let and enclosing the wanted variable names comma-separated in parentheses or directly using variable "." and the position of the tuple variable starting at 0.

Arrays in Rust have a fixed length and can only store variables of one type. Arrays are written as a comma-separated list inside square brackets. If the number of values to store is unclear, use another type, like a Vector from standard library. Arrays are stack allocated. Specifying the type of array is done by type semi-colon number of elements inside square brackets. "let a = [3; 5];" would create an array of size 5 and populated with the value 3.
Accessing items in an array is done like most other languages with "array_name[index]" with the position starting at 0. Accessing index outside of array's length will result in a panic program exit, not a compile-time error.

## 3 Functions
Rust does not care where you define your functions. The function can be defined to have parameters and when it does, you can provide it with concrete values called arguments for those parameters. If you define parameters in the function signature, you also declare the type of each parameter.

Statements are instructions that perform some action and do not return a value, Expressions evaluate to a resulting value.

Functions that evaluate to expressions end without semicolons, returning the last value. If you want return early you can use the "return" keyword.

For functions that returns values declare the type of the output in de function signature, using "->" and then the type to return.

## 4 Comments
Simple comments, comments that will be ignored when compiling or creating docs, begin with "// " and ends at the end of a line. You can also create multiline comments starting with "/*" and ending with "*/". Note that multiline doc-comments starts with "/**".

## 5 Control Flow
The "if" expression is like many other languages. Note that the condition does not have to be inside of parentheses, but the branch is to be inside of curly brackets. The condition must evaluate to a boolean and do so explicitly. You can combine "if" with "else if" and "else" to allow for multiple conditions. The program checks each expression in turn and only executes the first body for which the condition holds true, then stops checking the rest expressions.

While if statements are valuable, try to not clutter your code with them, think of where you might be able to use "match" instead for example.

You can use if expressions in let statements, simply write it inline ending with a semicolon, note that the result of the expression will have to return the same type in all branches of the expression or the compiler will error.

There are three kinds of loops in Rust: loop, while and for. The keyword "break" can be used to exit the loop.
