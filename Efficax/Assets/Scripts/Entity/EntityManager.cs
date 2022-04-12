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
    [SerializeField] private Entity entityPrefab;

    private Dictionary<uint, Entity> entities;

    private void Start()
    {
        entities = new Dictionary<uint, Entity>();
    }

    public void UpdateEntity(EntityUpdateData data)
    {
        Entity entity;
        if (!entities.ContainsKey(data.id))
        {
            entity = Instantiate(entityPrefab, Vector3.zero, Quaternion.identity, entitiesParent);
            entities.Add(data.id, entity);
        }
        else
        {
            entity = entities[data.id];
        }

        entity.transform.position = data.pos;
    }
}