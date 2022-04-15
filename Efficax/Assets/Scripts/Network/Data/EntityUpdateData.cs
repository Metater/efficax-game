using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityUpdateData : IReadData<EntityUpdateData>
{
    public uint id;
    public Vector2 pos;
    public byte inputSequence;

    public EntityUpdateData Read(NetDataReader reader)
    {
        id = reader.GetUInt();
        pos = DataUtils.ReadPos(reader);
        inputSequence = reader.GetByte();
        return this;
    }
}