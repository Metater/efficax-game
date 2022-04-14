using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private Rigidbody2D rb;

    [SerializeField] private float rotateLerp;

    private float desiredAngle = 0;

    private void Start()
    {
        print("created entity");
    }

    private void FixedUpdate()
    {
        rb.MoveRotation(Mathf.LerpAngle(transform.localEulerAngles.z, desiredAngle, rotateLerp));
    }

    public virtual void UpdateEnity(EntityUpdateData data)
    {
        Vector2 lastPos = rb.position;
        rb.MovePosition(data.pos);
        if (data.pos != lastPos)
        {
            Vector2 moveVector = data.pos - lastPos;
            desiredAngle = Vector2.SignedAngle(Vector2.up, moveVector);
        }
    }
}