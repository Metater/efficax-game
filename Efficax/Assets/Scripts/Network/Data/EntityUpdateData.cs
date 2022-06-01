using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityUpdateData : NetworkData, IReadData<EntityUpdateData>
{
    public ulong id;
    public Vector2 pos;
    public byte inputSequence;

    public EntityUpdateData Read(NetDataReader reader)
    {
        id = reader.GetULong();
        pos = DataUtils.ReadPos(reader);
        inputSequence = reader.GetByte();
        return this;
    }

    public EntityUpdateData SetTickId(byte tickId)
    {
        TickId = tickId;
        return this;
    }
}