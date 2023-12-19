function mergeArray(fe, myArray) {
    return new Promise((resolve, reject) => {
      var secondarray = [6, 7, 8, 9];
  
      const indexOfSix = secondarray.indexOf(6);
      secondarray.splice(indexOfSix, 0, fe);
      secondarray.push(...myArray);
  
      console.log("Modified secondarray:", secondarray);
  
      const sum = secondarray.reduce((acc, curr) => acc + curr, 0);
      console.log("Sum of elements in secondarray:", sum);
  
      if (sum > 30) {
        resolve(fetchImages(sum));
      } else {
        reject("Sum is less than or equal to 30");
      }
    });
  }
  
  function fetchImages(sum) {
    const imageCount = sum ; 
    const images = [];
  
    for (let i = 0; i < imageCount; i++) {
      images.push(`https://picsum.photos/200/300?random=${i}`);
    }
  
    return images;
  }
  
  
  (() => {
      var myArray = [1, 2, 3, 4, 5];
    let fe = myArray.shift();
    console.log(fe);
    console.log(myArray);
  
    mergeArray(fe, myArray)
      .then((images) => {
        console.log("Fetched images:", images);

      })
      .catch((error) => {
        console.error("Sum is not greater than 30:", error);
      });
  })();
  






  

// (function f1 () {
//     let arr = [1,2,3,4,5]
//     let fc = arr.shift();
//     f2(fc , ...arr)
// })();

// function f2(e , ...arr){
//     let arr2 = [6,7,8,9]
//     arr2.unshift(e)
//     arr2 = arr2.concat(...arr)

//     let promise = new Promise((resolve , reject) => {
//         let sum = arr2.reduce((e , sum)=>e + sum)
//         if(sum > 30){
//             resolve(sum);
//         }else{
//             reject();
//         }
//     })

//     promise.then((e)=>{
//         fetch(`https://jsonplaceholder.typicode.com/photos?_limit=${e}`)
//         .then(response => response.json())
//         .then(json => {
//             for(let i = 0 ; i< json.length ; i++){
//                 console.log(i+1 , json[i].url);
//             }
//         })
//     },()=>{
//         console.log("sum is not greater than 30")
//     })
// }









// nCr


// Function to calculate combination (nCr) using recursion
function combination(n, r) {
  if (r === 0 || r === n) {
    return 1;
  } else {
    return combination(n - 1, r - 1) + combination(n - 1, r);
  }
}

// Example usage:
const n = 5;
const r = 2;

console.log(`Combination (${n}C${r}): ${combination(n, r)}`);






// storing all element of array using recursssion


// Function to store words from arrays using recursion
function storeWords(arrays, currentIndex = 0, result = []) {
  if (currentIndex === arrays.length) {
    return result;
  } else {
    const currentArray = arrays[currentIndex];
    result.push(...currentArray);
    return storeWords(arrays, currentIndex + 1, result);
  }
}

// Example arrays
const array1 = ['apple', 'orange', 'banana'];
const array2 = ['carrot', 'broccoli', 'lettuce'];
const array3 = ['cat', 'dog', 'rabbit'];

// Storing words from arrays using recursion
const arrays = [array1, array2, array3];
const storedWords = storeWords(arrays);

console.log('Stored words from arrays:', storedWords);









//  nth fibonatchi not grater then 1000, take prime out and difference it and sum of all difference using rcursion

// Function to generate the Fibonacci series up to a limit
function generateFibonacci(limit, series = [0, 1]) {
  const nextNumber = series[series.length - 1] + series[series.length - 2];
  if (nextNumber > limit) {
    return series;
  }
  series.push(nextNumber);
  return generateFibonacci(limit, series);
}

// Function to check if a number is prime
function isPrime(num) {
  if (num <= 1) return false;
  if (num <= 3) return true;

  for (let i = 2; i <= Math.sqrt(num); i++) {
    if (num % i === 0) return false;
  }
  return true;
}

// Function to find prime numbers in a series
function findPrimes(series) {
  return series.filter((num) => isPrime(num));
}

// Function to calculate differences between consecutive prime numbers
function calculateDifferences(arr) {
  const differences = [];
  for (let i = 0; i < arr.length - 1; i++) {
    differences.push(arr[i + 1] - arr[i]);
  }
  return differences;
}

// Function to sum positive differences using recursion
function sumPositiveDifferences(arr, currentIndex = 0, sum = 0) {
  if (currentIndex === arr.length) {
    return sum;
  } else {
    const currentDiff = arr[currentIndex];
    if (currentDiff > 0) {
      sum += currentDiff;
    }
    return sumPositiveDifferences(arr, currentIndex + 1, sum);
  }
}

// Define the maximum limit for the Fibonacci series
const limit = 1000;

// Generate Fibonacci series
const fibonacciSeries = generateFibonacci(limit);

// Find prime numbers from the Fibonacci series
const primesInFibonacci = findPrimes(fibonacciSeries);

// Calculate differences between consecutive prime numbers
const differences = calculateDifferences(primesInFibonacci);

// Sum positive differences using recursion
const positiveSum = -[sumPositiveDifferences(differences)];

console.log('Fibonacci Series:', fibonacciSeries);
console.log('Prime Numbers in Fibonacci Series:', primesInFibonacci);
console.log('Differences between Consecutive Primes:', differences);
console.log('Sum of Positive Differences:', positiveSum);
