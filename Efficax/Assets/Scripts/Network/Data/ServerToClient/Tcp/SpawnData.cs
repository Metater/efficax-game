using UnityEngine;

public class SpawnData : NetworkData<SpawnData>
{
    public EntityType EntityType { get; private set; }
    public uint EntityId { get; private set; }
    public Vector2 Pos { get; private set; }

    public override SpawnData Read(NetDataReader reader)
    {
        EntityType = (EntityType)reader.GetByte();
        EntityId = reader.GetUInt();
        Pos = DataUtils.ReadPos(reader);

        return this;
    }
}
