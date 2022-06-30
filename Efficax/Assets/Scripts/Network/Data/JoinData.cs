using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JoinData : NetworkData<JoinData>
{
    public uint PlayerId { get; private set; }
    public Vector2 Pos { get; private set; }

    public override JoinData Read(NetDataReader reader)
    {
        PlayerId = reader.GetUInt();
        Pos = DataUtils.ReadPos(reader);

        return this;
    }
}
