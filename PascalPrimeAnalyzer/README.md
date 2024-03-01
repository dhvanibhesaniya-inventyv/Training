# Question :
## Develope a program that reads a number which is 4 digit long, then get combinations and find prime numbers from it , then get the highest prime number and make a pascle triangle using that number, get output in html and show  display the prime numbers in pascle triangle with underline.

#

# Code &  Explain :



This JavaScript code performs several operations based on user input and generates a Pascal's Triangle:
<br /> <br />


## generateDigitPermutations: 
This function generates all possible permutations of digits from the input number by converting it to a string, finding its unique permutations, and returning them as an array.

```javascript
function generateDigitPermutations(input) {
  const strInput = input.toString();
  const result = [];

  function permute(current, remainingDigits) {
    if (current.length > 0) {
      result.push(parseInt(current));
    }

    if (remainingDigits.length === 0) {
      return;
    }

    for (let i = 0; i < remainingDigits.length; i++) {
      const updatedCurrent = current + remainingDigits[i];
      const updatedRemaining =
        remainingDigits.slice(0, i) + remainingDigits.slice(i + 1);
      permute(updatedCurrent, updatedRemaining);
    }
  }

  permute("", strInput);

  const uniquePermutations = [...new Set(result)];
  return uniquePermutations;
}
```
<br /> <br />

## isPrime:
A utility function that checks whether a number is prime. It iterates from 2 up to the square root of the number and returns true if it's prime, false otherwise.


```javascript
function isPrime(num) {
  if (num <= 1) {
    return false;
  }
  for (let i = 2; i <= Math.sqrt(num); i++) {
    if (num % i === 0) {
      return false;
    }
  }
  return true;
}
```
<br /> <br />

## generatePascalsTriangle: 
This function generates Pascal's Triangle up to a specified number of rows (rows) using dynamic programming. It creates a 2D array representing the triangle and fills it with Pascal's Triangle values based on the maximum prime number.

```javascript
function generatePascalsTriangle(rows, maxPrime) {
  const triangle = [];
  for (let i = 0; i < rows; i++) {
    triangle[i] = new Array(i + 1);
    triangle[i][0] = 1;
    triangle[i][i] = 1;
    for (let j = 1; j < i; j++) {
      triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
      if (triangle[i][j] > maxPrime) {
        return triangle.slice(0, i);
      }
    }
  }
  return triangle;
}
```
<br /> <br />

The addEventListener function is assigned to the "Submit" button. It triggers the calculation and rendering process when the button is clicked.

It reads the user-input number and ensures it's valid (a number and at least 1000).

It searches for the maximum prime number less than or equal to the user-input number.

It renders Pascal's Triangle based on the maximum prime number, highlighting prime numbers in the triangle.

It calculates the error ratio by comparing the count of prime numbers in the triangle to the total count of prime numbers and displays it as a percentage in the HTML.

It finds and displays the prime numbers that are not present in the Pascal's Triangle.

The code involves handling user input, generating permutations, finding prime numbers, constructing Pascal's Triangle, and calculating and displaying the error ratio based on prime numbers present in the Pascal's Triangle compared to the total count of prime numbers.

<br /> <br />

```javascript
document.getElementById("submitButton").addEventListener("click", function () {
  let inputNumber = parseInt(document.getElementById("inputNumber").value);

  if (isNaN(inputNumber) || inputNumber < 1000) {
      alert("Please enter a valid number that is at least 1000.");
      return;
  }

  let maxPrime = -1;

  while (!isPrime(inputNumber)) {
      inputNumber--;
  }

  maxPrime = inputNumber;
  document.getElementById("inputNumber").value = inputNumber;

  document.getElementById("result").innerText = "Max Prime Number: " + maxPrime;

  const pascalsTriangle = generatePascalsTriangle(maxPrime, maxPrime);
  const primeNumbers = generateDigitPermutations(maxPrime.toString()).filter((number) => isPrime(number));

  const pascalDisplay = document.getElementById("pascalTriangle");
  pascalDisplay.innerHTML = "";

  for (let i = 0; i < pascalsTriangle.length; i++) {
      const row = document.createElement("div");
      row.classList.add("row");

      for (let j = 0; j < pascalsTriangle[i].length; j++) {
          const cell = document.createElement("span");
          const currentNumber = pascalsTriangle[i][j];

          if (primeNumbers.includes(currentNumber)) {
              cell.classList.add("prime");
          }

          const spaces = 4;
          cell.innerHTML = currentNumber + "&nbsp;".repeat(spaces);
          row.appendChild(cell);
      }

      pascalDisplay.appendChild(row);
  }

  document.getElementById("primeArray").innerText = "Prime Numbers: " + primeNumbers.join(', ');


  const primeNumbersInPascal = primeNumbers.filter(number => {
    return pascalsTriangle.flat().includes(number);
});

const errorRatio = ((primeNumbersInPascal.length / primeNumbers.length) * 100).toFixed(2);

// Displaying error ratio in HTML
  const errorDisplay = document.getElementById("errorRatio");
  errorDisplay.innerText = `Error Ratio: ${100- errorRatio}%`;
  // document.getElementById("result").innerText = "Max Prime Number: " + maxPrime;


  const primeNumbersNotInPascal = primeNumbers.filter(number => {
    return !pascalsTriangle.flat().includes(number);
});

document.getElementById("primeNotInPascal").innerText = "Prime Numbers Not in Pascal's Triangle: " + primeNumbersNotInPascal.join(', ');

});

function isSingleDigitPrime(num) {
  return num < 10 && isPrime(num);
}
```
#

# Output :
![Pascal triangle](https://github.com/dhvanibhesaniya-inventyv/Training/assets/153286337/b27f630d-6112-4c83-a336-3a39021ca36d)
#

# Author :

[DHVANI BHESANIYA . J](https://github.com/dhvanibhesaniya-inventyv/)

