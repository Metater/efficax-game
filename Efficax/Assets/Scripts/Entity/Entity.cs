using UnityEngine;

public abstract class Entity : MonoBehaviour
{
    public abstract void Init(uint tickId, Vector2 pos);
    public abstract void Snapshot(EntitySnapshotData data);
}