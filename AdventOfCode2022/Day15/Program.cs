using System.Numerics;
using Day15;

// Entry point for the program
var sol = new Solution("../../../Day15Puzzle.txt");
var part1 = sol.SolvePart1();
Console.WriteLine($"Part 1: {part1}"); // 5_142_231
var part2 = sol.SolvePart2();
Console.WriteLine($"Part 2: {part2}"); // 10_884_459_367_718

public sealed class Solution
{
    private readonly List<SensorAndBeacon> _sensorsAndBeacons;

    // Constructor creates the list of sensors and beacons from the input file
    public Solution(string filename)
    {
        var list = new List<SensorAndBeacon>();

        foreach (var line in File.ReadAllLines(filename))
        {
            var split = line.Split(' ');
            var sensorX = int.Parse(split[2][2..^1]);
            var sensorY = int.Parse(split[3][2..^1]);
            var sensor = new Point { X = sensorX, Y = sensorY };
            var beaconX = int.Parse(split[8][2..^1]);
            var beaconY = int.Parse(split[9][2..]);
            var beacon = new Point { X = beaconX, Y = beaconY };
            list.Add(new SensorAndBeacon { Sensor = sensor, Beacon = beacon });
        }

        _sensorsAndBeacons = list;
    }

    public int SolvePart1(int y = 2_000_000)
    {
        // First, add all beacons to hashset
        var beaconHashes = new HashSet<Point>();
        foreach (var sb in _sensorsAndBeacons)
        {
            beaconHashes.Add(sb.Beacon);
        }

        var pointHashes = new HashSet<Point>();
        foreach (var sb in _sensorsAndBeacons)
        {
            var points = sb.GetPointsInRow(y);
            foreach (var point in points)
            {
                // Add point if there is not a beacon at this location already
                if (!beaconHashes.Contains(point))
                    pointHashes.Add(point);
            }
        }

        return pointHashes.Count;
    }


    // Did not expect this dumb approach to actually work, but it does (and relatively quickly)
    public BigInteger SolvePart2(int min = 0, int max = 4_000_000)
    {
        var i = 1;
        // Just keep trying for as long as it takes
        while (i < int.MaxValue)
        {
            foreach (var sb in _sensorsAndBeacons)
            {
                // Where statement excludes points that have an x or y below the min (0) or above the max (4_000_000)
                var diamondPoints = sb.GetAllPointsAtGivenDistanceOutsideSensorRange(i)
                                      .Where(p => p.X >= min && p.X <= max && p.Y >= min && p.Y <= max);

                foreach (var point in diamondPoints)
                {
                    if (!IsPointInRangeOfAnySensor(point))
                    {
                        return PointToPart2Solution(point);
                    }
                }
            }

            i++;
        }

        return BigInteger.MinusOne;
    }

    private bool IsPointInRangeOfAnySensor(Point point)
    {
        return _sensorsAndBeacons.Any(sb => sb.IsPointInRange(point));
    }

    private static BigInteger PointToPart2Solution(Point point)
    {
        const int xMultiplier = 4_000_000;
        var xMultiplied = BigInteger.Multiply(point.X, xMultiplier);
        var res = BigInteger.Add(xMultiplied, point.Y);
        return res;
    }
}