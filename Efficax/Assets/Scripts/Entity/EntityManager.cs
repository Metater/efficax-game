using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityManager : MonoBehaviour
{
    [SerializeField] private Transform entitiesParent;
    [SerializeField] private Entity entityPrefab;

    private Dictionary<uint, Entity> entities;

    private void Awake()
    {
        entities = new();
    }

    private void Update()
    {
        ResetIfDisconnected();
    }

    private void ResetIfDisconnected()
    {
        if (GameManager.I.IsDisconnected)
        {
            foreach ((_, Entity entity) in entities)
            {
                Destroy(entity.gameObject);
            }
            entities.Clear();
        }
    }

    public bool TryGetEntity(uint entityId, out Entity entity)
    {
        return entities.TryGetValue(entityId, out entity);
    }

    public void Spawn(uint tickId, EntityType entityType, uint entityId, Vector2 pos)
    {
        if (!entities.ContainsKey(entityId))
        {
            Entity entity = Instantiate(entityPrefab, pos, Quaternion.identity, entitiesParent);
            entities.Add(entityId, entity);
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
        if (entities.TryGetValue(entityId, out Entity entity))
        {
            entities.Remove(entityId);
            Destroy(entity.gameObject);
        }
        else
        {
            throw new Exception($"Cannot despawn entity, it does not exist");
        }
    }

    public void Snapshot(EntitySnapshotData data)
    {
        if (entities.TryGetValue(data.Id, out Entity entity))
        {
            entity.Snapshot(data);
        }
    }
}