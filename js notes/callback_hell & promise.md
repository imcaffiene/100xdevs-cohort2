
### Difference between setTimeout and setInterval
- setTimeout will only run once
- setInterval will run forever until you tell it to stop

### Callbacks functions
- A callback function is a function passed into another function as an argument, which is then invoked inside the outer function to complete some kind of routine or action.

- Advantages of callbacks:
    - We can call a function when an another function finishes it's execution.
    - We can pass a function as an argument to another function.
    - We can create a function inside another function.

- For example : If we have to create an ecommerce website and we have to create an order, proceed to payment, pay and deliver the product. We can use callback function to do this, because we can't proceed to payment until we create an order, we can't pay until we proceed to payment and we can't deliver until we pay for the product.

```js
const cars = ['ferrari', 'lambo', 'porsche', 'tesla'];

api.createOrder(cars, function(){
    api.proceedToPayment(function(){
        api.pay(function(){
            api.deliver(function(){
                console.log('done');
            })
        })
    })
})
```

- So this is a callback hell, and it's not good. Callback hell is when you have a lot of nested callbacks, and it's hard to read and maintain.

- After callback hell our code starts horizontally instead of vertically and because of this structure it is also called pyramid of doom, because it looks like a pyramid.

- another rissue of callback is invesion of control - we are giving control to the api or programm, we are not in control of the flow of the program anymore.

### Promises
- A promise is an object that may produce a single value some time in the future: either a resolved value, or a reason that it’s not resolved (e.g., a network error occurred). A promise may be in one of 3 possible states: fulfilled, rejected, or pending.

- Promises are a way to write asynchronous code that is easier to read, easier to write and easier to debug.

- Advantage of promises:
    - We can call a function when an another function finishes it's execution.
    - We can pass a function as an argument to another function.
    - We can create a function inside another function.
    - We can chain multiple promises together.
    - We can catch errors in a single place.
    - We can use async/await to write synchronous looking code.


