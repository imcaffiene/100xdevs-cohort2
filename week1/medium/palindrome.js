/*
  Implement a function `isPalindrome` which takes a string as argument and returns true/false as its result.
  Note: the input string is case-insensitive which means 'Nan' is a palindrom as 'N' and 'n' are considered case-insensitive.
*/

function isPalindrome(str) {
  // Convert the string to lowercase for case-insensitivity
  const lowerCaseStr = str.toLowerCase();

  // Remove non-alphanumeric characters
  const cleanStr = lowerCaseStr.replace(/[^a-z0-9]/g, '');

  // Check if the cleaned string is the same forwards and backwards
  return cleanStr === cleanStr.split('').reverse().join('');
}

module.exports = isPalindrome;
