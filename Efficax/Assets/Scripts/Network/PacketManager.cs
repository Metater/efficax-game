using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading;
using System;
using UnityEngine;

public class PacketManager : MonoBehaviour
{
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

        AddTCPPacketHandlers();
        AddUDPPacketHandlers();
    }

    public void ExecuteQueuedUpdates()
    {
        ExecuteActions(UpdateQueue);
    }

    public void ExecuteQueuedFixedUpdates()
    {
        ExecuteActions(FixedUpdateQueue);
    }

    public void Handle(NetDataReader reader, bool isTcp, uint tickId)
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

    private void AddTCPPacketHandlers()
    {
        tcpHandlers[Network.Join] = PacketHandler.Create(this, PacketHandlerType.Update, (JoinData data) =>
        {
            GameManager.I.playerManager.Joined(data);
            GameManager.I.entityManager.Spawn(data.TickId, EntityType.Player, data.PlayerId, data.Pos);
        });
        tcpHandlers[Network.Spawn] = PacketHandler.Create(this, PacketHandlerType.Update, (SpawnData data) =>
        {
            GameManager.I.entityManager.Spawn(data.TickId, data.EntityType, data.EntityId, data.Pos);
        });
        tcpHandlers[Network.Despawn] = PacketHandler.Create(this, PacketHandlerType.Update, (DespawnData data) =>
        {
            GameManager.I.entityManager.Despawn(data.EntityId);
        });
    }

    private void AddUDPPacketHandlers()
    {
        udpHandlers[Network.Snapshot] = PacketHandler.Create(this, PacketHandlerType.Update, (SnapshotData data) =>
        {
            foreach (EntitySnapshotData entityUpdate in data.EntitySnapshots)
            {
                GameManager.I.entityManager.Snapshot(entityUpdate);
            }
        });
    }
}
