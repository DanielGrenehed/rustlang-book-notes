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
