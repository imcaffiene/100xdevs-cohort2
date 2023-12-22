/*
  Implement a function `countVowels` that takes a string as an argument and returns the number of vowels in the string.
  Note: Consider both uppercase and lowercase vowels ('a', 'e', 'i', 'o', 'u').

  Once you've implemented the logic, test your code by running
*/

function countVowels(str) {
  // Convert the string to lowercase to make the count case-insensitive
  const lowerCaseStr = str.toLowerCase();

  // Define an array of vowels
  const vowels = ['a', 'e', 'i', 'o', 'u'];

  // Use the filter method to create an array of vowels in the string
  const vowelArray = lowerCaseStr.split('').filter(char => vowels.includes(char));

  // Return the count of vowels
  return vowelArray.length;
}

module.exports = countVowels;