using System;

class CurrencyConverter
{
    static void Main()
    {
        Console.WriteLine("Enter 'rs' to convert rupees to paisa or 'paisa' to convert paisa to rupees:");
        string choice = Console.ReadLine();

        if (choice.ToLower() == "rs")
        {
            Console.WriteLine("Enter the amount in rupees:");
            double rupees = Convert.ToDouble(Console.ReadLine());

            double paisa = rupees * 100; 
            Console.WriteLine("Converted amount in paisa: " + paisa + " paisa");
        }
        else if (choice.ToLower() == "paisa")
        {
            Console.WriteLine("Enter the amount in paisa:");
            double paisa = Convert.ToDouble(Console.ReadLine());

            double rupees = paisa / 100; 
            Console.WriteLine("Converted amount in rupees: " + rupees + " rupees");
        }
        else
        {
            Console.WriteLine("Invalid Input! Please enter 'rs' or 'paisa'.");
        }
    }
}
