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

    public void Handle(NetDataReader reader, bool isTcp, byte tickId)
    {
        byte packetType = reader.GetByte();
        switch (packetType)
        {
            case 2:
                HandleTickUpdate(reader, tickId);
                break;
            default:
                print($"Unknown packet type: {packetType}");
                break;
        }
    }

    private void HandleTickUpdate(NetDataReader reader, byte tickId)
    {
        TickUpdateData data = new TickUpdateData().SetTickId(tickId).Read(reader);

        updateQueue.Enqueue(() => {
            foreach (EntityUpdateData entityUpdate in data.entityUpdates)
            {
                gameManager.entityManager.UpdateEntity(entityUpdate);
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
