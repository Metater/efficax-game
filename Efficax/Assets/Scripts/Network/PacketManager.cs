using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading;
using System;
using UnityEngine;

public class PacketManager : MonoBehaviour
{
    [SerializeField] private GameManager gameManager;

    public ConcurrentQueue<Action> UpdateQueue { get; private set; }
    public ConcurrentQueue<Action> FixedUpdateQueue { get; private set; }

    private PacketHandler[] tcpHandlers;
    private PacketHandler[] udpHandlers;

    private void Awake()
    {
        UpdateQueue = new ConcurrentQueue<Action>();
        FixedUpdateQueue = new ConcurrentQueue<Action>();

        tcpHandlers = new PacketHandler[256];
        udpHandlers = new PacketHandler[256];

        tcpHandlers[NetworkData.Join] = PacketHandler.Create(this, PacketHandlerType.Update, (JoinData data) =>
        {
            gameManager.playerManager.SetPlayerId(data.PlayerId);
        });

        udpHandlers[NetworkData.Snapshot] = PacketHandler.Create(this, PacketHandlerType.Update, (SnapshotData data) =>
        {
            foreach (EntitySnapshotData entityUpdate in data.EntitySnapshots)
            {
                gameManager.entityManager.EntitySnapshot(entityUpdate);
            }
        });
    }

    public void ExecuteQueuedUpdates()
    {
        ExecuteActions(UpdateQueue);
    }

    public void ExecuteQueuedFixedUpdates()
    {
        ExecuteActions(FixedUpdateQueue);
    }

    public void Handle(NetDataReader reader, bool isTcp, byte tickId)
    {
        byte packetType = reader.GetByte();

        if (isTcp)
        {
            var handler = tcpHandlers[packetType];
            if (handler is not null)
            {
                handler.Handle(reader, tickId);
            }
            else
            {
                print($"TCP Unknown packet type: {packetType}");
            }
        }
        else
        {
            var handler = udpHandlers[packetType];
            if (handler is not null)
            {
                handler.Handle(reader, tickId);
            }
            else
            {
                print($"UDP Unknown packet type: {packetType}");
            }
        }
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
