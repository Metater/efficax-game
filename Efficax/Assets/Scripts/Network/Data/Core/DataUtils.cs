using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class DataUtils
{
    public static void WritePos(NetDataWriter writer, Vector2 pos)
    {
        writer.Put(ScaleFloatToUInt(-256, 256, Mathf.Clamp(pos.x, -256, 256)));
        writer.Put(ScaleFloatToUInt(-256, 256, Mathf.Clamp(pos.y, -256, 256)));
    }
    public static Vector2 ReadPos(NetDataReader reader)
    {
        return new Vector2(UnscaleUIntToFloat(-256, 256, reader.GetUShort()), UnscaleUIntToFloat(-256, 256, reader.GetUShort()));
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