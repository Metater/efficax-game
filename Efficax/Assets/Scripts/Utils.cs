using UnityEngine;

public static class Utils
{
    public static double TickToSeconds(uint tick)
    {
        return tick * 0.04;
    }

    public static double InverseLerpDouble(double t, double a, double b)
    {
        return (t - a) / (b - a);
    }

    public static double StepTowardsDouble(double from, double to, double by)
    {
        double value = from;

        if (from == to)
        {
            return value;
        }

        if (from > to)
        {
            value = from - by;
            if (from < to)
            {
                value = to;
            }
        }

        if (from < to)
        {
            value = from + by;
            if (from > to)
            {
                value = to;
            }
        }

        return value;
    }

    public static uint ScaleFloatToUInt(float lower, float upper, float value)
    {
        float step = Mathf.InverseLerp(lower, upper, value);
        return (uint)Mathf.Lerp(0, 65535, step);
    }
    public static byte ScaleFloatToByte(float lower, float upper, float value)
    {
        float step = Mathf.InverseLerp(lower, upper, value);
        return (byte)Mathf.Lerp(0, 255, step);
    }

    public static float UnscaleUIntToFloat(float lower, float upper, uint value)
    {
        float step = Mathf.InverseLerp(0, 65536, value);
        return Mathf.Lerp(lower, upper, step);
    }
    public static float UnscaleByteToFloat(float lower, float upper, byte value)
    {
        float step = Mathf.InverseLerp(0, 255, value);
        return Mathf.Lerp(lower, upper, step);
    }
}