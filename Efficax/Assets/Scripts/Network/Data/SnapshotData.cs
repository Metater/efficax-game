using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class SnapshotData : NetworkData<SnapshotData>
{
    public List<EntitySnapshotData> EntitySnapshots { get; private set; } = new();

    public override SnapshotData Read(NetDataReader reader)
    {
        byte entitySnapshotCount = reader.GetByte();
        for (int i = 0; i < entitySnapshotCount; i++)
        {
            EntitySnapshots.Add(new EntitySnapshotData().SetTickIdAndRead(reader, TickId));
        }

        return this;
    }
}

public class EntitySnapshotData : NetworkData<EntitySnapshotData>
{
    // Entity Snapshot Data Enum Variants
    public const byte None = 0;
    public const byte Player = 1;

    public uint Id { get; private set; }
    public Vector2 Pos { get; private set; }
    public IEntitySpecificSnapshotData Data { get; private set; }

    public override EntitySnapshotData Read(NetDataReader reader)
    {
        Id = reader.GetUInt();
        Pos = DataUtils.ReadPos(reader);

        byte type = reader.GetByte();
        Data = type switch
        {
            None => null,
            Player => new PlayerSnapshotData().SetTickIdAndRead(reader, TickId),
            _ => null,
        };

        return this;
    }
}

public class PlayerSnapshotData : NetworkData<PlayerSnapshotData>, IEntitySpecificSnapshotData
{
    public byte InputSequence { get; private set; }

    public EntityType GetEntityType()
    {
        return EntityType.Player;
    }

    public override PlayerSnapshotData Read(NetDataReader reader)
    {
        InputSequence = reader.GetByte();

        return this;
    }
}