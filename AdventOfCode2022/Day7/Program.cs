
namespace Day7;

public class Program
{
    // Very messy and inefficient solutions to both parts, but it works
    private static void Main(string[] args)
    {
        var solutions = Solutions("Day7Puzzle.txt");
        Console.WriteLine("Solution to part 1: " + solutions.part1);  // 1077191
        Console.WriteLine("Solution to part 2: " + solutions.part2);  // 5649896
    }


    public static (decimal part1, decimal part2) Solutions(string filename)
    {
        var lines = File.ReadAllLines(filename);

        // Store all directories (as strings) and their files in a dictionary
        var dirDict = new Dictionary<string, HashSet<DirFile>>();

        // Collect the files in the current directory
        var curFilesSet = new HashSet<DirFile>();
        var curDir = "/";
        foreach (var line in lines.Skip(1))
        {
            if (!line.StartsWith("$ cd"))
            {
                var split = line.Split(" ");
                // If the line starts with a number, it's a file
                // Example:  1234 file.txt
                // The parse will fail otherwise, and the loop will continue
                if (int.TryParse(split[0], out var fileSize))
                {
                    var file = new DirFile { Name = split[1], Size = fileSize };
                    curFilesSet.Add(file);
                }
            }
            else
            {
                // Probably won't happen but just in case the directory was put 
                // in before it was ls'd and then added again later after it was ls'd
                if (!dirDict.ContainsKey(curDir) || dirDict[curDir].Count < curFilesSet.Count)
                {
                    dirDict[curDir] = curFilesSet;
                }
                // Line would be something like: '$ cd ..' or '$ cd newdir'
                var newDir = line.Split(" ")[2];
                if (newDir == "..")
                {
                    // Example of line below: makes ab/ce/vrf/vt/ into ab/ce/vrf/   (removes last slash and dir name)
                    // ... There is definitely a better way to do this
                    curDir = curDir.Substring(0, curDir.LastIndexOf('/', curDir.LastIndexOf('/') - 1)) + '/';
                }
                else
                {
                    // Turns /dir1/dir2/ into /dir1/dir2/newdir/
                    curDir = $"{curDir}{newDir}/";
                }
                // Reset the files set now that we're in a new directory
                curFilesSet = new HashSet<DirFile>();
            }
        }
        // There may be one last directory that wasn't added to the dict yet
        if (!dirDict.ContainsKey(curDir) || dirDict[curDir].Count < curFilesSet.Count)
            dirDict[curDir] = curFilesSet;

        var dirAndSize = CountSizeOfEachDirectory(dirDict);

        var spaceTakenOnDisk = GetSpaceTakenOnDisk(filename);
        const decimal AllAvailableSpace = 70000000;
        const decimal SpaceNeeded = 30000000;
        decimal freeSpace = AllAvailableSpace - spaceTakenOnDisk;
        decimal spaceToFree = SpaceNeeded - freeSpace;

        Console.WriteLine($"Total space used on disk: {spaceTakenOnDisk}, Space free: {freeSpace}");
        Console.WriteLine($"Need to free up {spaceToFree}");

        // For part 2, find the smallest directory that is greater than or equal to the space needed
        var part2Solution = decimal.MaxValue;
        foreach (var (key, value) in dirAndSize)
        {
            if (value >= spaceToFree)
            {
                if (value < part2Solution)
                {
                    part2Solution = value;
                }
            }
        }

        // For part 1, sum up all the files that are less than 100000
        decimal part1Solution = 0;
        foreach (var (dir, size) in dirAndSize)
        {
            if (size < 100000)
            {
                part1Solution += size;
            }
        }
        return (part1Solution, part2Solution);
    }


    // Count the size of each directory, this is the sum of all the files in that directory and all subdirectories
    public static Dictionary<string, decimal> CountSizeOfEachDirectory(Dictionary<string, HashSet<DirFile>> dict)
    {
        var newDict = new Dictionary<string, decimal>();
        foreach (var dir in dict.Keys)
        {
            decimal curSum = 0;
            foreach (var (otherDir, files) in dict)
            {
                // Either is the same directory or the other directory is a subdirectory of the current directory
                if (otherDir.Contains(dir))
                {
                    foreach (var file in files)
                    {
                        curSum += file.Size;
                    }
                }
            }
            newDict.Add(dir, curSum);
        }
        return newDict;
    }

    // Get the total space taken on disk by storing all the files in a HashSet
    // and then summing up the sizes of all the files
    public static decimal GetSpaceTakenOnDisk(string filename)
    {
        var lines = File.ReadAllLines(filename);
        var fileSet = new HashSet<DirFile>();
        foreach (var line in lines)
        {
            var split = line.Split(" ");
            if (int.TryParse(split[0], out var fileSize))
            {
                var file = new DirFile { Name = split[1], Size = fileSize };
                fileSet.Add(file);
            }
        }
        var totalSize = 0M;
        foreach (var file in fileSet)
        {
            totalSize += file.Size;
        }
        return totalSize;
    }


    // File class to store the name and size of a file
    public class DirFile
    {
        public string? Name { get; set; }
        public int Size { get; set; }

        public override int GetHashCode()
        {
            // Trying to make sure that the hashcode is unique (enough) for each file
            // Could still have a collision if there are two files with the same name and size
            return HashCode.Combine(Name, Size);
        }
    }
}