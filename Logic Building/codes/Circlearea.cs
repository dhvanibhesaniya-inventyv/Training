using System;

namespace CircleArea
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Enter the radius of the circle: ");
            float radius = float.Parse(Console.ReadLine());

            float area = CalculateArea(radius);
            Console.WriteLine($"The area of the circle is {area}.");
        }

        static float CalculateArea(float radius)
        {
            return (float)(Math.PI * radius * radius);
        }
    }
}