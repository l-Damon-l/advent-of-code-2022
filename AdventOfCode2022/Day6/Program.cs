namespace Day6;

internal class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine($"Part 1 Solution: {Solution(4)}");    // 1651
        Console.WriteLine($"Part 2 Solution: {Solution(14)}");   // 3837
    }

    public static int Solution(int distinctCharsRequired)
    {
        const string file = "Day6Puzzle.txt";
        var line = File.ReadAllLines(file)[0];
        for (int i = 0; i + distinctCharsRequired < line.Length; i++)
        {
            var curSet = line[i..(i + distinctCharsRequired)].ToHashSet();
            if (curSet.Count == distinctCharsRequired)
            {
                return i + distinctCharsRequired;
            }
        }
        return 0;
    }
}
