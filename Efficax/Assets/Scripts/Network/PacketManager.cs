using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading;
using System;
using UnityEngine;

public class PacketManager : MonoBehaviour
{
    [SerializeField] private GameManager gameManager;

    private ConcurrentQueue<Action> updateQueue;
    private ConcurrentQueue<Action> fixedUpdateQueue;

    private void Awake()
    {
        updateQueue = new ConcurrentQueue<Action>();
        fixedUpdateQueue = new ConcurrentQueue<Action>();
    }

    public void ExecuteQueuedUpdates()
    {
        ExecuteActions(updateQueue);
    }

    public void ExecuteQueuedFixedUpdates()
    {
        ExecuteActions(fixedUpdateQueue);
    }

    public void Handle(NetDataReader reader, bool isTcp, byte tickId)
    {
        byte packetType = reader.GetByte();

        if (isTcp)
        {
            switch (packetType)
            {
                case NetworkData.Join:
                    HandleJoin(reader, tickId);
                    break;
                default:
                    print($"TCP Unknown packet type: {packetType}");
                    break;
            }
        }
        else
        {
            switch (packetType)
            {
                case NetworkData.Snapshot:
                    HandleSnapshot(reader, tickId);
                    break;
                default:
                    print($"UDP Unknown packet type: {packetType}");
                    break;
            }
        }
    }

    private void HandleJoin(NetDataReader reader, byte tickId)
    {
        JoinData data = new JoinData().Read(reader, tickId);

        updateQueue.Enqueue(() =>
        {
            gameManager.playerManager.SetPlayerId(data.PlayerId);
        });
    }

    private void HandleSnapshot(NetDataReader reader, byte tickId)
    {
        SnapshotData data = new SnapshotData().Read(reader, tickId);

        updateQueue.Enqueue(() =>
        {
            foreach (EntitySnapshotData entityUpdate in data.EntitySnapshots)
            {
                gameManager.entityManager.EntitySnapshot(entityUpdate);
            }
        });
    }

    private static void ExecuteActions(ConcurrentQueue<Action> actions)
    {
        int actionsCount = actions.Count;
        for (int i = 0; i < actionsCount; i++)
        {
            if (actions.TryDequeue(out Action action))
                action();
            else
                break;
        }
    }
}
