using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityManager : MonoBehaviour
{
    // Unity
    [SerializeField] private Transform entitiesParent;
    [SerializeField] private Entity entityPrefab;

    // Public state
    public Dictionary<uint, Entity> Entities { get; private set; }

    private void Awake()
    {
        Entities = new();

        GameManager.I.packetManager.AddTcpHandler(Network.ServerToClient.Tcp.Spawn, PacketHandlerType.Update, (SpawnData data) =>
        {
            Spawn(data.TickId, data.EntityType, data.EntityId, data.Pos);
        });
        GameManager.I.packetManager.AddTcpHandler(Network.ServerToClient.Tcp.Despawn, PacketHandlerType.Update, (DespawnData data) =>
        {
            Despawn(data.EntityId);
        });

        GameManager.I.packetManager.AddUdpHandler(Network.ServerToClient.Udp.Snapshot, PacketHandlerType.Update, (SnapshotData data) =>
        {
            foreach (EntitySnapshotData entityUpdate in data.EntitySnapshots)
            {
                Snapshot(entityUpdate);
            }
        });

        GameManager.I.OnDisconnected += () =>
        {
            foreach (Entity entity in Entities.Values)
            {
                Destroy(entity.gameObject);
            }
            Entities.Clear();
        };
    }

    public void Spawn(uint tickId, EntityType entityType, uint entityId, Vector2 pos)
    {
        if (!Entities.ContainsKey(entityId))
        {
            Entity entity = Instantiate(entityPrefab, pos, Quaternion.identity, entitiesParent);
            Entities.Add(entityId, entity);
            entity.Init(pos);
            entity.RawSnapshot(tickId, pos);
        }
        else
        {
            throw new Exception($"Entity exists already when spawing: type: {entityType}");
        }
    }

    public void Despawn(uint entityId)
    {
        if (Entities.TryGetValue(entityId, out Entity entity))
        {
            Entities.Remove(entityId);
            Destroy(entity.gameObject);
        }
        else
        {
            throw new Exception($"Cannot despawn entity, it does not exist");
        }
    }

    public void Snapshot(EntitySnapshotData data)
    {
        if (Entities.TryGetValue(data.Id, out Entity entity))
        {
            entity.Snapshot(data);
        }
    }
}