using System;

class Numbers {
    static void Main(string[] args) {
        int n = int.Parse(Console.ReadLine()); 

        for (int i = 1; i <= n; i++) {
            Console.WriteLine($"{i}"); 
        }
    }
}


// using System;

// class Program
// {
//     static void Main()
//     {
//         int n = 5;  
//         int product = 1;

//         for (int i = 1; i <= n; i++)
//         {
//             product *= i;
//         }

//         Console.WriteLine("The product of the first " + n + " natural numbers is: " + product);
//     }
// }


// using System;

// class Program
// {
//     static void Main()
//     {
//         int n = 5;  
//         int sum = 0;

//         for (int i = 1; i <= n; i++)
//         {
//             sum += i;
//         }

//         Console.WriteLine("The sum of the first " + n + " natural numbers is: " + sum);
//     }
// }
