namespace Day15;

public class SensorAndBeacon
{
    public Point Sensor { get; init; }
    public Point Beacon { get; init; }

    private int Distance => Math.Abs(Sensor.X - Beacon.X) + Math.Abs(Sensor.Y - Beacon.Y);

    public bool IsPointInRange(Point point)
    {
        var distanceFromSensor = Math.Abs(Sensor.X - point.X) + Math.Abs(Sensor.Y - point.Y);
        return distanceFromSensor <= Distance;
    }


    // For part 1
    // Given a y value (row number), gets all points in the row that are in range of the sensor (if any)
    public List<Point> GetPointsInRow(int y)
    {
        var points = new List<Point>();
        var yDiff = Math.Abs(Sensor.Y - y);

        // The area covered by the sensor does not include y, so return an empty list
        if (yDiff > Distance)
        {
            return points;
        }

        // The area covered by the sensor includes y
        // Add all points from left to right that are in range (Could be only one)
        var xDiff = Distance - yDiff;
        var leftX = Sensor.X - xDiff;
        var rightX = Sensor.X + xDiff;
        for (var x = leftX; x <= rightX; x++)
        {
            points.Add(new Point(x, y));
        }

        return points;
    }


    // For part 2
    // Returns all the points outside the sensor's range (forms a diamond shape)
    public List<Point> GetAllPointsAtGivenDistanceOutsideSensorRange(int unitsOutsideRange)
    {
        var dist = Distance + unitsOutsideRange;
        var points = new List<Point>();

        // Get top point of diamond shape
        var topY = Sensor.Y - dist;
        var curPoint = new Point(Sensor.X, topY);

        // From top to right of diamond
        while (curPoint.Y <= Sensor.Y)
        {
            points.Add(curPoint);
            curPoint = new Point(curPoint.X + 1, curPoint.Y + 1);
        }

        // From right to bottom of diamond
        while (curPoint.X >= Sensor.X)
        {
            points.Add(curPoint);
            curPoint = new Point(curPoint.X - 1, curPoint.Y + 1);
        }

        // From bottom to left of diamond
        while (curPoint.Y >= Sensor.Y)
        {
            points.Add(curPoint);
            curPoint = new Point(curPoint.X - 1, curPoint.Y - 1);
        }

        // From left to top of diamond
        while (curPoint.X <= Sensor.X)
        {
            points.Add(curPoint);
            curPoint = new Point(curPoint.X + 1, curPoint.Y - 1);
        }

        return points;
    }
}