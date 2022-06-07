using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityManager : MonoBehaviour
{
    [SerializeField] private GameManager gameManager;

    [SerializeField] private Transform entitiesParent;
    [SerializeField] private GameObject entityPrefab;

    private Dictionary<ulong, Entity> entities;

    private void Awake()
    {
        entities = new();
    }

    private void Update()
    {
        if (gameManager.IsDisconnected)
        {
            foreach ((ulong _, Entity entity) in entities)
            {
                Destroy(entity.gameObject);
            }
            entities.Clear();
        }
    }

    public Entity GetEntity(ulong entityId)
    {
        return entities[entityId];
    }

    public void EntitySnapshot(EntitySnapshotData data)
    {
        Entity entity;
        if (!entities.ContainsKey(data.Id))
        {
            entity = Instantiate(entityPrefab, data.Pos, Quaternion.identity, entitiesParent).GetComponent<Entity>();
            entities.Add(data.Id, entity);
            entity.Init(gameManager);
        }
        else
        {
            entity = entities[data.Id];
        }
        entity.UpdateEnity(data);
    }
}