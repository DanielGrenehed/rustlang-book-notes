# 4 Understanding Ownership

## 1 What is Ownership?
Rust handles memory management at compile time, through ownership with a set of rules that are check by the compiler. This way there is no garbage collector slowing down performance at runtime.

### The stack and heap
    Data that is stored on the stack must have a known and fixed size, if the size is to be unknown at compile time, make a heap allocation instead. The heap allocates space for data and returns a pointer to the data adress. Pushing to the stack is faster than putting it on the heap. The stack is generally faster since there is no pointer hopping.

    Local variables of functions and values passed into functions(including heap data pointers) get pushed onto the stack and popped off when the function is over.
