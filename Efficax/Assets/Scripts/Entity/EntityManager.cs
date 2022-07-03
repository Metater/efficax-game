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

    public void Joined(JoinData data)
    {
        if (!entities.ContainsKey(data.PlayerId))
        {
            Entity entity = Instantiate(entityPrefab, data.Pos, Quaternion.identity, entitiesParent);
            entities.Add(data.PlayerId, entity);
            entity.Init();
            // TODO update once
        }
        else
        {
            throw new Exception("Entity exists already when joining");
        }
    }

    public void EntitySnapshot(EntitySnapshotData data)
    {
        Entity entity;
        if (!entities.ContainsKey(data.Id))
        {
            entity = Instantiate(entityPrefab, data.Pos, Quaternion.identity, entitiesParent);
            entities.Add(data.Id, entity);
            entity.Init();
        }
        else
        {
            entity = entities[data.Id];
        }
        entity.UpdateEnity(data);
    }
}