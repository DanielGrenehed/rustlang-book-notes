# 4: Understanding Ownership

## 1. What is Ownership?
Rust handles memory management at compile time, through ownership with a set of rules that are check by the compiler. This way there is no garbage collector slowing down performance at runtime.

### The stack and heap
Data that is stored on the stack must have a known and fixed size, if the size is to be unknown at compile time, make a heap allocation instead. The heap allocates space for data and returns a pointer to the data adress. Pushing to the stack is faster than putting it on the heap. The stack is generally faster since there is no pointer hopping. Local variables of functions and values passed into functions(including heap data pointers) get pushed onto the stack and popped off when the function is over.

Ownership rules:
* All values have an owner variable.
* There is only one owner at a time.
* Values are dropped when the owner goes out of scope.

Rust calls drop for all stack variables in a scope when the scope runs out.
Variables like Strings, that have an undefined length at compile time are allocated on the heap and pointed to from stack variables. When you set one variable to another that is stack allocated, the new variable will point to the same heap allocated memory as the other variable, you will have to pointers to one set of data. This makes Rust faster, since the data is not copied, but can make things complicated for the uninformed.

When creating a copy of a heap pointer Rust invalidates the first pointer, to prevent a double free error. The operation of making a copy of a pointer in Rust is called a "move".

You can create a deep copy of the heap data by using ".clone()", but know that this may be expensive.

The copy trait is for types that contains simple scalar values and does not require any heap allocation.

Since variables you pass to functions will be dopped unless passed back, we can either return tuples, or pass references instead of the actual variable.

## 2. References and Borrowing
Using references(&) instead of variables are one way of not moving actual data when calling a function. We can use "*" to dereference references. References passed into functions are not dropped since the function is not the owner of the variable. The act of having references as function parameters i called borrowing. References are immutable by default as usual variables, so if you want to be able to modify the referred data you must use mutable references.

Note that you can only have one mutable reference to a piece of data in a scope. You can only let one person borrow your book at a time. This restriction will help and prevent the computer form having multiple pointers access the same data at the same time. We can have multiple mutable references, just not simultaneously in the same scope. We can have many immutable references to one variable at a time as long as we don't have a mutable reference to the same variable, since the immutable references expect the value to not change.

The rust compiler does not allow you to create a dangling pointer, a pointer to a variable that is deleted.

## 3. The Slice Type
A string slice is a reference to a part of a string. "&s[0..5]" Would get a reference to the first 5 characters of the string s, if you start at the beginning of the string you can just leave the zero out, and likewise if you want to the end of the string you can just omit the last number. Going by the same logic, you can get a slice of the entire string by doing "&s[..]". Slices are written as a "&str" type. Slices are made up of two pointers, start and end.
Slices are like immutable references, so after creating a slice of a variable you cannot create a mutable reference to that variable.
Using slices as parameters are a better way of passing strings since they can take both string literals and a full slice of a string.

You can make a slice of an array too.
