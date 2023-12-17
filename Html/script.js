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

