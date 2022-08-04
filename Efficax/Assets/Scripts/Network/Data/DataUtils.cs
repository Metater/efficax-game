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
        writer.Put(Utils.ScaleFloatToUInt(-256, 256, Mathf.Clamp(pos.x, -256, 256)));
        writer.Put(Utils.ScaleFloatToUInt(-256, 256, Mathf.Clamp(pos.y, -256, 256)));
    }
    public static Vector2 ReadPos(NetDataReader reader)
    {
        return new Vector2(Utils.UnscaleUIntToFloat(-256, 256, reader.GetUShort()), Utils.UnscaleUIntToFloat(-256, 256, reader.GetUShort()));
    }
}