namespace Day7.test
{
    public class Day7Test
    {

        [Test]
        public void Test1() {
            var day7TestSolutions = Program.Solutions("../../../Day7TestPuzzle.txt");
            Assert.That(day7TestSolutions.part1, Is.EqualTo(95437M));
        }


        [Test]
        public void Test2() {
            var day7TestSolutions = Program.Solutions("../../../Day7TestPuzzle.txt");
            Assert.That(day7TestSolutions.part2, Is.EqualTo(24933642M));
        }
    }
}