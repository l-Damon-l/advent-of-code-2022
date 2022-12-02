using System.Reflection;

namespace Day2;

internal class Program
{
    static void Main(string[] args) {
        Console.WriteLine($"Part 1: {Solution.ReadFileForSolutionPart1()}");  // 8392
        Console.WriteLine($"Part 2: {Solution.ReadFileForSolutionPart2()}");  // 10116
    }
}

public enum RPS
{
    Rock,
    Paper,
    Scissors
}

public class RpsChoice
{
    public RPS Choice { get; }
    public int ChoiceScore { get; }
    public RpsChoice(char choice) {
        Choice = choice switch {
            'A' or 'X' => RPS.Rock,
            'B' or 'Y' => RPS.Paper,
            'C' or 'Z' => RPS.Scissors,
            _ => throw new ArgumentException(nameof(choice)),
        };
        ChoiceScore = Choice switch {
            RPS.Rock => 1,
            RPS.Paper => 2,
            RPS.Scissors => 3,
            _ => throw new ArgumentException(nameof(choice)),
        };
    }
}

public class RpsChoicePartTwoThem
{
    public RPS Choice { get; }
    public int ChoiceScore { get; }
    public RpsChoicePartTwoThem(char choice) {
        Choice = choice switch {
            'A' => RPS.Rock,
            'B' => RPS.Paper,
            'C' => RPS.Scissors,
            _ => throw new ArgumentException(nameof(choice)),
        };
        ChoiceScore = Choice switch {
            RPS.Rock => 1,
            RPS.Paper => 2,
            RPS.Scissors => 3,
            _ => throw new ArgumentException(nameof(choice)),
        };
    }
}

public class RpsChoicePartTwoUs
{
    public RPS Choice { get; }
    public int ChoiceScore { get; }
    public RpsChoicePartTwoUs(RPS theirChoice, char choice) {
        Choice = choice switch {
            // We want to lose
            'X' => theirChoice switch {
                RPS.Rock => RPS.Scissors,
                RPS.Paper => RPS.Rock,
                RPS.Scissors => RPS.Paper,
                _ => throw new ArgumentException(nameof(theirChoice)),
            },
            // We want to draw
            'Y' => theirChoice,
            // We want to win
            'Z' => theirChoice switch {
                RPS.Rock => RPS.Paper,
                RPS.Paper => RPS.Scissors,
                RPS.Scissors => RPS.Rock,
                _ => throw new ArgumentException(nameof(theirChoice)),
            },

            _ => throw new ArgumentException(nameof(choice)),
        };
        ChoiceScore = Choice switch {
            RPS.Rock => 1,
            RPS.Paper => 2,
            RPS.Scissors => 3,
            _ => throw new ArgumentException(nameof(choice)),
        };
    }
}

public static class RpsGame
{
    public static int Play(RpsChoice theirChoice, RpsChoice ourChoice) {
        const int WinScore = 6;
        const int LoseScore = 0;
        const int DrawScore = 3;
        return theirChoice.Choice switch {
            RPS.Rock => ourChoice.Choice switch {
                RPS.Rock => ourChoice.ChoiceScore + DrawScore,
                RPS.Paper => ourChoice.ChoiceScore + WinScore,
                RPS.Scissors => ourChoice.ChoiceScore + LoseScore,
                _ => throw new ArgumentException(nameof(ourChoice)),
            },
            RPS.Paper => ourChoice.Choice switch {
                RPS.Rock => ourChoice.ChoiceScore + LoseScore,
                RPS.Paper => ourChoice.ChoiceScore + DrawScore,
                RPS.Scissors => ourChoice.ChoiceScore + WinScore,
                _ => throw new ArgumentException(nameof(ourChoice)),
            },
            RPS.Scissors => ourChoice.Choice switch {
                RPS.Rock => ourChoice.ChoiceScore + WinScore,
                RPS.Paper => ourChoice.ChoiceScore + LoseScore,
                RPS.Scissors => ourChoice.ChoiceScore + DrawScore,
                _ => throw new ArgumentException(nameof(ourChoice)),
            },
            _ => throw new ArgumentException(nameof(theirChoice)),
        };
    }

    // Same as above just with new class parameters
    public static int PlayPartTwo(RpsChoicePartTwoThem theirChoice, RpsChoicePartTwoUs ourChoice) {
        const int WinScore = 6;
        const int LoseScore = 0;
        const int DrawScore = 3;
        return theirChoice.Choice switch {
            RPS.Rock => ourChoice.Choice switch {
                RPS.Rock => ourChoice.ChoiceScore + DrawScore,
                RPS.Paper => ourChoice.ChoiceScore + WinScore,
                RPS.Scissors => ourChoice.ChoiceScore + LoseScore,
                _ => throw new ArgumentException(nameof(ourChoice)),
            },
            RPS.Paper => ourChoice.Choice switch {
                RPS.Rock => ourChoice.ChoiceScore + LoseScore,
                RPS.Paper => ourChoice.ChoiceScore + DrawScore,
                RPS.Scissors => ourChoice.ChoiceScore + WinScore,
                _ => throw new ArgumentException(nameof(ourChoice)),
            },
            RPS.Scissors => ourChoice.Choice switch {
                RPS.Rock => ourChoice.ChoiceScore + WinScore,
                RPS.Paper => ourChoice.ChoiceScore + LoseScore,
                RPS.Scissors => ourChoice.ChoiceScore + DrawScore,
                _ => throw new ArgumentException(nameof(ourChoice)),
            },
            _ => throw new ArgumentException(nameof(theirChoice)),
        };
    }
}

public static class Solution
{
    public static int ReadFileForSolutionPart1() {
        const string filename = "../../../../Day2/Day2Puzzle.txt";

        var ourScore = 0;
        using var reader = new StreamReader(filename);
        while (!reader.EndOfStream) {
            var line = reader.ReadLine();
            var theirChoice = new RpsChoice(line[0]);
            var ourChoice = new RpsChoice(line[2]);
            ourScore += RpsGame.Play(theirChoice, ourChoice);
        }
        return ourScore;
    }

    public static int ReadFileForSolutionPart2() {
        const string filename = "../../../../Day2/Day2Puzzle.txt";

        var ourScore = 0;
        using var reader = new StreamReader(filename);
        while (!reader.EndOfStream) {
            var line = reader.ReadLine();
            var theirChoice = new RpsChoicePartTwoThem(line[0]);
            var ourChoice = new RpsChoicePartTwoUs(theirChoice.Choice, line[2]);
            ourScore += RpsGame.PlayPartTwo(theirChoice, ourChoice);
        }
        return ourScore;
    }
}
