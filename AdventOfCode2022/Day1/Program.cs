internal class Program
{
    private static void Main(string[] args) {
        Console.WriteLine($"Solution to part 1 is {Day1.Solution().part1}");
        Console.WriteLine($"Solution to part 2 is {Day1.Solution().part2}");
    }
}

internal class Day1
{
    public static (int part1, int part2) Solution() {
        var elfList = CreateElfListFromFile();
        var mostCalories = 0;
        var secondMostCalories = 0;
        var thirdMostCalories = 0;

        // Dumb approach, but it works fine for this.
        for (int i = 0; i < elfList.Count; i++) {
            var calories = elfList[i].NumCalories;
            if (calories > mostCalories) {
                thirdMostCalories = secondMostCalories;
                secondMostCalories = mostCalories;
                mostCalories = calories;
                continue;
            }
            if (calories > secondMostCalories) {
                thirdMostCalories = secondMostCalories;
                secondMostCalories = calories;
                continue;
            }
            if (calories > thirdMostCalories) {
                thirdMostCalories = calories;
            }
        }
        var sumOfThreeMost = mostCalories + secondMostCalories + thirdMostCalories;
        return (mostCalories, sumOfThreeMost);
    }

    public static List<Elf> CreateElfListFromFile() {
        const string filename = "../../../../Day1/Day1Puzzle.txt";

        var allElves = new List<Elf>();
        var curElfCalories = new List<int>();
        using var reader = new StreamReader(filename);
        while (!reader.EndOfStream) {
            var line = reader.ReadLine();
            if (line is not null && line.Length > 0) {
                var numFromLine = int.TryParse(line.Trim(), out var num);
                if (numFromLine) {
                    curElfCalories.Add(num);
                }
            }
            else {
                // blank/null line
                allElves.Add(new Elf(curElfCalories));
                // make a new list for the next elf
                curElfCalories = new List<int>();
            }

        }
        return allElves;
    }

    internal class Elf
    {
        public List<int> CaloryList { get; init; }
        public Elf(List<int> caloryList) {
            CaloryList = caloryList;
        }
        public int NumCalories => CaloryList.Sum();
    }
}