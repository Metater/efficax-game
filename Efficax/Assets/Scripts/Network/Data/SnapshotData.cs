using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class SnapshotData : NetworkData, IReadData<SnapshotData>
{
    public List<EntitySnapshotData> EntitySnapshots { get; private set; } = new();

    public SnapshotData Read(NetDataReader reader, byte tickId)
    {
        TickId = tickId;

        byte entitySnapshotCount = reader.GetByte();
        for (int i = 0; i < entitySnapshotCount; i++)
        {
            EntitySnapshots.Add(new EntitySnapshotData().Read(reader, tickId));
        }

        return this;
    }
}

public class EntitySnapshotData : NetworkData
{
    public ulong Id { get; private set; }
    public Vector2 Pos { get; private set; }
    public EntitySpecificSnapshotData Data { get; private set; }

    public EntitySnapshotData Read(NetDataReader reader, byte tickId)
    {
        TickId = tickId;

        Id = reader.GetULong();
        Pos = DataUtils.ReadPos(reader);

        EntityType entityType = (EntityType)reader.GetByte();
        Data = EntitySpecificSnapshotData.Read(entityType, reader, tickId);

        return this;
    }
}

public abstract class EntitySpecificSnapshotData : NetworkData
{
    public EntityType Type { get; protected set; }

    public static EntitySpecificSnapshotData Read(EntityType entityType, NetDataReader reader, byte tickId)
    {
        EntitySpecificSnapshotData data = entityType switch
        {
            EntityType.Player => new PlayerSnapshotData().Read(reader, tickId),
            _ => null,
        };

        if (data is not null)
        {
            data.Type = entityType;
        }

        return data;
    }
}

public class PlayerSnapshotData : EntitySpecificSnapshotData, IReadData<PlayerSnapshotData>
{
    public byte InputSequence { get; private set; }

    public PlayerSnapshotData Read(NetDataReader reader, byte tickId)
    {
        TickId = tickId;

        InputSequence = reader.GetByte();

        return this;
    }
}