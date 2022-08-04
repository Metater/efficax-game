using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading;
using System;
using UnityEngine;

public class PacketManager
{
    // Private state
    private readonly PacketHandler[] tcpHandlers = new PacketHandler[256];
    private readonly PacketHandler[] udpHandlers = new PacketHandler[256];

    private readonly ConcurrentQueue<Action> updateQueue = new();
    private readonly ConcurrentQueue<Action> fixedUpdateQueue = new();

    #region Action Lifecycle
    public void EnqueueUpdate(Action update) => updateQueue.Enqueue(update);
    public void EnqueueFixedUpdate(Action fixedUpdate) => fixedUpdateQueue.Enqueue(fixedUpdate);

    private static void ExecuteActions(ConcurrentQueue<Action> actions)
    {
        int actionsCount = actions.Count;
        for (int i = 0; i < actionsCount; i++)
        {
            if (actions.TryDequeue(out Action action))
            {
                action();
            }
            else
            {
                break;
            }
        }
    }

    public void ExecuteUpdates() => ExecuteActions(updateQueue);
    public void ExecuteFixedUpdates() => ExecuteActions(fixedUpdateQueue);
    #endregion Action Lifecycle

    #region Handling
    public void HandleTcp(NetDataReader reader, uint tickId)
    {
        byte packetType = reader.GetByte();

        var handler = tcpHandlers[packetType];
        if (handler is not null)
        {
            handler.Handle(reader, tickId);
        }
        else
        {
            Debug.Log($"TCP Unknown packet type: {packetType}");
        }
    }
    public void HandleUdp(NetDataReader reader, uint tickId)
    {
        byte packetType = reader.GetByte();

        var handler = udpHandlers[packetType];
        if (handler is not null)
        {
            handler.Handle(reader, tickId);
        }
        else
        {
            Debug.Log($"UDP Unknown packet type: {packetType}");
        }
    }

    public void AddTcpHandler<T>(byte index, PacketHandlerType handlerType, Action<T> handler) where T : NetworkData<T>, new()
    {
        #if UNITY_EDITOR
        if (tcpHandlers[index] is not null)
        {
            throw new Exception($"Duplicate TCP handler, index {index}");
        }
        #endif

        tcpHandlers[index] = PacketHandler.Create<T>(this, handlerType, handler);
    }
    public void AddUdpHandler<T>(byte index, PacketHandlerType handlerType, Action<T> handler) where T : NetworkData<T>, new()
    {
        #if UNITY_EDITOR
        if (udpHandlers[index] is not null)
        {
            throw new Exception($"Duplicate UDP handler, index {index}");
        }
        #endif

        udpHandlers[index] = PacketHandler.Create<T>(this, handlerType, handler);
    }
    #endregion Handling
}

public class PacketHandler
{
    private readonly Action<NetDataReader, uint> packetHandler;

    private PacketHandler(Action<NetDataReader, uint> packetHandler)
    {
        this.packetHandler = packetHandler;
    }

    public static PacketHandler Create<T>(PacketManager packetManager, PacketHandlerType handlerType, Action<T> handler) where T : NetworkData<T>, new()
    {
        void PacketHandler(NetDataReader reader, uint tickId)
        {
            T data = new T().SetTickIdAndRead(reader, tickId);
            switch (handlerType)
            {
                case PacketHandlerType.Default:
                    handler(data);
                    break;
                case PacketHandlerType.Update:
                    packetManager.EnqueueUpdate(() => handler(data));
                    break;
                case PacketHandlerType.FixedUpdate:
                    packetManager.EnqueueFixedUpdate(() => handler(data));
                    break;
            }
        }

        return new PacketHandler(PacketHandler);
    }


    public void Handle(NetDataReader reader, uint tickId)
    {
        packetHandler(reader, tickId);
    }
}

public enum PacketHandlerType
{
    Default,
    Update,
    FixedUpdate
}