using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private Rigidbody2D rb;

    private void Start()
    {
        print("created entity");
    }

    public virtual void UpdateEnity(Vector2 pos)
    {
        rb.MovePosition(pos);
    }
}