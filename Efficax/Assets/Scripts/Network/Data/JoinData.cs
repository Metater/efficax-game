using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JoinData : NetworkData, IReadData<JoinData>
{
    public ulong PlayerId { get; private set; }

    public JoinData Read(NetDataReader reader, byte tickId)
    {
        TickId = tickId;

        PlayerId = reader.GetULong();

        return this;
    }
}
