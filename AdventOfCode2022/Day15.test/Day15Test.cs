using System.Numerics;

namespace Day15.test;

public class Tests
{
    [Test]
    public void AocTest()
    {
        var sol = new Solution("../../../../Day15/Day15TestPuzzle.txt");
        Assert.That(sol.SolvePart1(10), Is.EqualTo(26));
    }

    [Test]
    public void AocTest2()
    {
        var sol = new Solution("../../../../Day15/Day15TestPuzzle.txt");
        Assert.That(sol.SolvePart2(0, 20), Is.EqualTo((BigInteger)56000011));
    }
}