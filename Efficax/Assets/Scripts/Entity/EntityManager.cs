using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityManager : MonoBehaviour
{
    public GameManager gameManager;

    [SerializeField] private Transform entitiesParent;
    [SerializeField] private GameObject entityPrefab;

    private Dictionary<ulong, Entity> entities;

    private void Awake()
    {
        entities = new Dictionary<ulong, Entity>();
    }

    public void UpdateEntity(EntityUpdateData data)
    {
        Entity entity;
        if (!entities.ContainsKey(data.id))
        {
            entity = Instantiate(entityPrefab, data.pos, Quaternion.identity, entitiesParent).GetComponent<Entity>();
            entities.Add(data.id, entity);
        }
        else
        {
            entity = entities[data.id];
        }
        entity.UpdateEnity(data);
    }
}