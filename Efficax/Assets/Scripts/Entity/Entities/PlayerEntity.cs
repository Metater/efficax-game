using UnityEngine;

public class PlayerEntity : Entity
{
    // Unity
    [SerializeField] private EntityInterpolator interpolator;

    // Unity Debug
    [SerializeField] private bool hasDebugPacketLoss;
    [Range(0, GameManager.TicksPerSecond)] [SerializeField] private float debugPacketsLostPerSecond;

    public override void Init(uint tickId, Vector2 pos)
    {
        interpolator.Init(tickId, pos);
    }

    public override void Snapshot(EntitySnapshotData data)
    {
        var snapshot = data.AsPlayerSnapshot;

        #if UNITY_EDITOR
        if (hasDebugPacketLoss)
        {
            if (Random.Range(0f, 100f) < (debugPacketsLostPerSecond / GameManager.TicksPerSecond) * 100f)
            {
                return;
            }
        }
        #endif

        interpolator.FeedPosition(snapshot.TickId, snapshot.Pos);
    }
}