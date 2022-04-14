using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading;
using System;
using UnityEngine;

public class PacketManager : MonoBehaviour
{
    public GameManager gameManager;

    private ConcurrentQueue<Action> updateQueue;
    private ConcurrentQueue<Action> fixedUpdateQueue;

    private void Awake()
    {
        updateQueue = new ConcurrentQueue<Action>();
        fixedUpdateQueue = new ConcurrentQueue<Action>();
    }

    private void Start()
    {
        
    }

    private void Update()
    {
        
    }

    public void ExecuteQueuedUpdates()
    {
        ExecuteActions(updateQueue);
    }

    public void ExecuteQueuedFixedUpdates()
    {
        ExecuteActions(fixedUpdateQueue);
    }

    public void Handle(NetDataReader reader)
    {
        byte packetType = reader.GetByte();
        switch (packetType)
        {
            case 2:
                HandleEntityUpdate(reader);
                break;
            default:
                print($"Unknown packet type: {packetType}");
                break;
        }
    }

    private void HandleEntityUpdate(NetDataReader reader)
    {
        EntityUpdateData data = new EntityUpdateData().Read(reader);
        fixedUpdateQueue.Enqueue(() => {
            gameManager.entityManager.UpdateEntity(data);
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
