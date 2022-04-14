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

    private void Awake()
    {

    }

    private void Start()
    {
        print("created entity");
    }

    private void FixedUpdate()
    {
        // TODO Eventually set rb.MovePosition every frame, interpolation for rbs only works between fixed updates
        rb.MoveRotation(Mathf.LerpAngle(transform.localEulerAngles.z, desiredAngle, rotateLerp));
    }

    public virtual void UpdateEnity(EntityUpdateData data)
    {
        UpdatePosition(data.pos);
    }

    private void UpdatePosition(Vector2 pos)
    {
        Vector2 lastPos = rb.position;
        rb.MovePosition(pos);
        if (pos != lastPos)
        {
            Vector2 moveVector = pos - lastPos;
            desiredAngle = Vector2.SignedAngle(Vector2.up, moveVector);
        }
    }
}