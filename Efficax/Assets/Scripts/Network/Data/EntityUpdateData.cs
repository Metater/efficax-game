using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityUpdateData : IReadData<EntityUpdateData>
{
    public ulong id;
    public Vector2 pos;
    public byte inputSequence;

    public EntityUpdateData Read(NetDataReader reader)
    {
        id = reader.GetULong();
        pos = DataUtils.ReadPos(reader);
        //pos = new Vector2(reader.GetFloat(), reader.GetFloat());
        inputSequence = reader.GetByte();
        return this;
    }
}