# Compiled Language vs Interpreted Language

### Compiled Language
1. First need to compile the code then execute it.
2. Compiled language is faster than interpreted language because it is compiled first.
3. Usually don't compile if there is an error in the code.
4. e.g. C, C++, C#, Java, Go, Rust, Swift, etc.

### Interpreted Language
1. Usually interpreted language is executed line by line.
2. Interpreted language is slower than compiled language because it is executed line by line.
3. Can execute the code even if there is an error in the code.
4. e.g. JavaScript, Python, Ruby, PHP, etc.

### Why Js is better than other languages
 - Js is a single threaded language.
 -  Js is a interpreted language.
-  Js is a dynamic language.
-  Js is a weakly typed language.
-  Js is a prototype based language.
-  Js is a functional language.


### Strict vs Dynamic language
- Strict language is a language where we have to declare the type of the variable before using it.
- Dynamic language is a language where we don't have to declare the type of the variable before using it.

### Single Threaded vs Multi Threaded
- Single threaded language is a language where only one task can be executed at a time.
- Multi threaded language is a language where multiple task can be executed at a time.

### What is the callback function
 - A callback function is a function passed into another function as an argument, which is then invoked inside the outer function to complete some kind of routine or action.
 e.g.
 ```js
    function greeting(name) {
        alert('Hello ' + name);
    }
 ```

# advantage of callback function
- Callbacks are a great way to handle something after something else has been completed.
- Callbacks allow us to pass functions as an argument to be executed later.
- Callbacks allow us to control the order in which functions are run.
e.g.
```js
    function greeting(name) {
        alert('Hello ' + name);
    }
    function processUserInput(callback) {
        var name = prompt('Please enter your name.');
        callback(name);
    }
    processUserInput(greeting);
```
### disadvantage of callback function
- Callbacks can be messy.
- Callback hell.
- Callbacks can be confusing.
- Callbacks can be hard to debug.

# What is the event loop
- The event loop is a mechanism that makes sure that the code in the call stack is executed in order and that the call stack does not get blocked by other processes.
- The event loop is a single threaded mechanism that is responsible for managing the call stack, checking if the call stack is empty and pushing new functions onto the call stack.
e.g.
```js
    function greeting(name) {
        alert('Hello ' + name);
    }
    function processUserInput(callback) {
        var name = prompt('Please enter your name.');
        callback(name);
    }
    processUserInput(greeting);
```
# What is the call stack
- The call stack is a mechanism that keeps track of the functions that are currently being executed and the functions that are waiting to be executed.
- The call stack is a LIFO (Last In, First Out) data structure.
- The call stack is a single threaded mechanism that is responsible for executing the functions pushed onto it.

# What is the event queue
- The event queue is a mechanism that keeps track of the functions that are ready to be executed.
- The event queue is a FIFO (First In, First Out) data structure.
- The event queue is a single threaded mechanism that is responsible for pushing functions onto the call stack.


