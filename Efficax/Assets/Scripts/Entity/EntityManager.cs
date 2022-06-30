using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityManager : MonoBehaviour
{
    [SerializeField] private Transform entitiesParent;
    [SerializeField] private GameObject entityPrefab;

    private Dictionary<uint, Entity> entities;

    private void Awake()
    {
        entities = new();
    }

    private void Update()
    {
        if (GameManager.I.IsDisconnected)
        {
            foreach ((uint _, Entity entity) in entities)
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

    public void EntitySnapshot(EntitySnapshotData data)
    {
        Entity entity;
        if (!entities.ContainsKey(data.Id))
        {
            entity = Instantiate(entityPrefab, data.Pos, Quaternion.identity, entitiesParent).GetComponent<Entity>();
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