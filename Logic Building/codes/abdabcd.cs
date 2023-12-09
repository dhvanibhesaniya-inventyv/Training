using System;

class MaxValue
{
    static void Main()
    {
        int a = 10;   
        int b = 15;  
        int c = 12;  

        if (a == b && b == c)
        {
            Console.WriteLine("All three values are equal: " + a);
        }
        else if (a >= b && a >= c)
        {
            Console.WriteLine("The maximum value is: " + a);
        }
        else if (b >= a && b >= c)
        {
            Console.WriteLine("The maximum value is: " + b);
        }
        else
        {
            Console.WriteLine("The maximum value is: " + c);
        }
    }
}




// using System;

// class MaxValue
// {
//     static void Main()
//     {
//         int a = 10;  
//         int b = 15;  
//         int c = 12;  
//         int d = 15;  

//         if (a == b && b == c && c == d)
//         {
//             Console.WriteLine("All four values are equal: " + a);
//         }
//         else if (a >= b && a >= c && a >= d)
//         {
//             Console.WriteLine("The maximum value is: " + a);
//         }
//         else if (b >= a && b >= c && b >= d)
//         {
//             Console.WriteLine("The maximum value is: " + b);
//         }
//         else if (c >= a && c >= b && c >= d)
//         {
//             Console.WriteLine("The maximum value is: " + c);
//         }
//         else
//         {
//             Console.WriteLine("The maximum value is: " + d);
//         }
//     }
// }
