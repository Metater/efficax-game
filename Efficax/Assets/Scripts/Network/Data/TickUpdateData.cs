﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class TickUpdateData : IReadData<TickUpdateData>
{
    public byte tickId;
    public List<EntityUpdateData> entityUpdates = new List<EntityUpdateData>();

    public TickUpdateData Read(NetDataReader reader)
    {
        tickId = reader.GetByte();
        byte entityUpdateCount = reader.GetByte();
        for (int i = 0; i < entityUpdateCount; i++)
        {
            entityUpdates.Add(new EntityUpdateData().Read(reader));
        }
        return this;
    }
}