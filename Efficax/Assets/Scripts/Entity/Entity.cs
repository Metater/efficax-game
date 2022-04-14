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

    public virtual void UpdateEnity(EntityUpdateData data)
    {
        rb.MovePosition(data.pos);
        if (data.rotation != 0)
        {
            float step = Mathf.InverseLerp(1, 9, data.rotation);
            float angle = Mathf.Lerp(0, -360, step);
            transform.localEulerAngles = new Vector3(0, 0, angle);
        }
    }
}