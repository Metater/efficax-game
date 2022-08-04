using UnityEngine;

public abstract class Entity : MonoBehaviour
{
    // Unity
    [SerializeField] private EntityInterpolator interpolator;

    // Unity Debug
    [SerializeField] private bool hasDebugPacketLoss;
    [Range(0, GameManager.TicksPerSecond)] [SerializeField] private float debugPacketsLostPerSecond;

    public virtual void Init(uint tickId, Vector2 pos)
    {
        interpolator.Init(tickId, pos);
    }

    public virtual void Snapshot(EntitySnapshotData data)
    {
        #if UNITY_EDITOR
        if (hasDebugPacketLoss)
        {
            if (Random.Range(0f, 100f) < (debugPacketsLostPerSecond / GameManager.TicksPerSecond) * 100f)
            {
                return;
            }
        }
        #endif

        interpolator.FeedPosition(data.TickId, data.Pos);
    }
}