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
    public uint Id { get; private set; }
    public EntityType Type { get; private set; }
    public object Data { get; private set; }

    public PlayerSnapshotData AsPlayerSnapshot => Data as PlayerSnapshotData;

    public override EntitySnapshotData Read(NetDataReader reader)
    {
        Id = reader.GetUInt();
        Type = (EntityType)reader.GetByte();

        Data = Type switch
        {
            EntityType.Player => new PlayerSnapshotData().SetTickIdAndRead(reader, TickId),
            _ => null,
        };

        return this;
    }
}

public class PlayerSnapshotData : NetworkData<PlayerSnapshotData>
{
    public Vector2 Pos { get; private set; }
    public byte InputSequence { get; private set; }

    public override PlayerSnapshotData Read(NetDataReader reader)
    {
        Pos = DataUtils.ReadPos(reader);
        InputSequence = reader.GetByte();

        return this;
    }
}