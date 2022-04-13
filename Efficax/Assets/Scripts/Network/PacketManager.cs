using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading;
using System;
using UnityEngine;

public class PacketManager : MonoBehaviour
{
    public GameManager gameManager;

    private ConcurrentQueue<Action> actions;

    private void Awake()
    {
        actions = new ConcurrentQueue<Action>();
    }

    private void Start()
    {
        
    }

    private void Update()
    {
        
    }

    public void ExecuteQueuedActions()
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
        actions.Enqueue(() => {
            gameManager.entityManager.UpdateEntity(data);
        });
    }
}
