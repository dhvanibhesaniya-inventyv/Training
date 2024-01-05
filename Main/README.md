# Question :  Develope a program that reads a number which is 4 digit long, then get combinations and find prime numbers from it , then geththe highest prime number and make a pascle triangle using that number, get output in html and show  display the prime numbers in pascle traangle with underline.


## Code :


## Explain :

This JavaScript code performs several operations based on user input and generates a Pascal's Triangle:

generateDigitPermutations: This function generates all possible permutations of digits from the input number by converting it to a string, finding its unique permutations, and returning them as an array.

isPrime: A utility function that checks whether a number is prime. It iterates from 2 up to the square root of the number and returns true if it's prime, false otherwise.

generatePascalsTriangle: This function generates Pascal's Triangle up to a specified number of rows (rows) using dynamic programming. It creates a 2D array representing the triangle and fills it with Pascal's Triangle values based on the maximum prime number.

The addEventListener function is assigned to the "Submit" button. It triggers the calculation and rendering process when the button is clicked.

It reads the user-input number and ensures it's valid (a number and at least 1000).

It searches for the maximum prime number less than or equal to the user-input number.

It renders Pascal's Triangle based on the maximum prime number, highlighting prime numbers in the triangle.

It calculates the error ratio by comparing the count of prime numbers in the triangle to the total count of prime numbers and displays it as a percentage in the HTML.

It finds and displays the prime numbers that are not present in the Pascal's Triangle.

The code involves handling user input, generating permutations, finding prime numbers, constructing Pascal's Triangle, and calculating and displaying the error ratio based on prime numbers present in the Pascal's Triangle compared to the total count of prime numbers.


### Author :

DHVANI BHESANIYA .J

