using UnityEngine;

public class DespawnData : NetworkData<DespawnData>
{
    public uint EntityId { get; private set; }

    public override DespawnData Read(NetDataReader reader)
    {
        EntityId = reader.GetUInt();

        return this;
    }
}
