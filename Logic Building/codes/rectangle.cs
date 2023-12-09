using System;

class Rectangle
{
    static void Main()
    {
        double length = 5.0;  
        double width = 3.0;   

        double area = length * width;
        double perimeter = 2 * (length + width);

        Console.WriteLine("The area of the rectangle is: " + area);
        Console.WriteLine("The perimeter of the rectangle is: " + perimeter);
    }
}
