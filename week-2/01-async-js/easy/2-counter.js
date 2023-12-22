// ## Counter without setInterval

// Without using setInterval, try to code a counter in Javascript. There is a hint at the bottom of the file if you get stuck.

let count = 0;
const increment = () => {
  count++;
  console.log(count);
  setTimeout(increment, 1000);
};