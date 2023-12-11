using System;

class PrintPattern
{
    static void Main()
    {
        int rows = 5;  // Replace this with the number of rows you want to print

        for (int i = 1; i <= rows; i++)
        {
            for (int j = 1; j <= i; j++)
            {
                Console.Write(j + " ");
            }
            Console.WriteLine();
        }
    }
}
