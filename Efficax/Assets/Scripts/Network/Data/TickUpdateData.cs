using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class TickUpdateData : NetworkData, IReadData<TickUpdateData>
{
    public List<EntityUpdateData> entityUpdates = new();

    public TickUpdateData Read(NetDataReader reader)
    {
        byte entityUpdateCount = reader.GetByte();
        for (int i = 0; i < entityUpdateCount; i++)
        {
            entityUpdates.Add(new EntityUpdateData().SetTickId(TickId).Read(reader));
        }
        return this;
    }

    public TickUpdateData SetTickId(byte tickId)
    {
        TickId = tickId;
        return this;
    }
}