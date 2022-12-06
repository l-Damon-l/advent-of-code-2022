using System.Text;

namespace Day5
{
    internal class Program
    {
        static void Main(string[] args)
        {
            var stacks = SetUpStacks();
            // MoveStacksPart1(stacks);   // MQSHJMWNH
            MoveStacksPart2(stacks);      // LLWJRBHVZ
            Console.WriteLine(GetStackTops(stacks));
        }


        public static Stack<char>[] SetUpStacks()
        {
            const string filename = "../Day5/Day5Puzzle.txt";
            var stacks = new Stack<char>[9];
            for (int i = 0; i < 9; i++)
            {
                stacks[i] = new Stack<char>();
            }

            var allLines = File.ReadAllLines(filename);
            for (var i = 7; i >= 0; i--)
            {
                var line = allLines[i];
                var curStack = 0;
                for (var j = 1; j < line.Length; j += 4)
                {
                    if (char.IsLetter(line[j]))
                    {
                        stacks[curStack].Push(line[j]);
                    }
                    curStack++;
                }
            }
            return stacks;
        }

        public static void MoveStacksPart1(Stack<char>[] stacks)
        {
            const string filename = "../Day5/Day5Puzzle.txt";
            var allLines = File.ReadAllLines(filename);
            var linePos = 10;
            while (linePos < allLines.Length)
            {
                var line = allLines[linePos];
                var splitLine = line.Split(' ');
                (int move, int from, int to) solTuple = (int.Parse(splitLine[1]), int.Parse(splitLine[3]), int.Parse(splitLine[5]));
                while (solTuple.move > 0)
                {
                    // - 1 as stacks are shown as 1 to 9 but are indexed as 0 to 8 in the array                    
                    var fromStack = stacks[solTuple.from - 1];
                    var toStack = stacks[solTuple.to - 1];
                    toStack.Push(fromStack.Pop());
                    solTuple.move--;
                }
                linePos++;
            }
        }

        public static void MoveStacksPart2(Stack<char>[] stacks)
        {
            const string filename = "../Day5/Day5Puzzle.txt";

            var allLines = File.ReadAllLines(filename);
            var linePos = 10;
            while (linePos < allLines.Length)
            {
                var line = allLines[linePos];
                var splitLine = line.Split(' ');
                (int move, int from, int to) solTuple = (int.Parse(splitLine[1]), int.Parse(splitLine[3]), int.Parse(splitLine[5]));

                // Use an extra stack so that the items can be added back
                // into the stack in the opposite order to part 1
                var extraStack = new Stack<char>();
                while (solTuple.move > 0)
                {
                    extraStack.Push(stacks[solTuple.from - 1].Pop());
                    solTuple.move--;
                }
                while (extraStack.Count > 0)
                {
                    stacks[solTuple.to - 1].Push(extraStack.Pop());
                }
                linePos++;
            }
        }

        public static void PrintStacks(Stack<char>[] stacks)
        {
            foreach (var stack in stacks)
            {
                foreach (var item in stack)
                {
                    Console.Write(item + " ");
                }
                Console.WriteLine();
            }
        }

        public static string GetStackTops(Stack<char>[] stacks)
        {
            var sb = new StringBuilder();
            foreach (var stack in stacks)
            {
                sb.Append(stack.Peek());
            }
            return sb.ToString();
        }
    }
}


