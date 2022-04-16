using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private Rigidbody2D rb;
    [SerializeField] private GameObject sprite;

    [SerializeField] private float rotateLerp;

    private Queue<float> desiredAngleQueue;

    private void Awake()
    {
        desiredAngleQueue = new Queue<float>(new float[] { 0 });
    }

    private void Start()
    {
        print("created entity");
    }

    private void Update()
    {
        //float time = Time.time;
        //transform.position = (new Vector2(Mathf.Sin(time) * 6, 0));

        //print(transform.position.x + " : " + time);
    }

    private void FixedUpdate()
    {
        // TODO Eventually set rb.MovePosition every frame, interpolation for rbs only works between fixed updates
        while (desiredAngleQueue.Count > 2)
        {
            desiredAngleQueue.Dequeue();
        }
        rb.MoveRotation(Mathf.LerpAngle(transform.localEulerAngles.z, desiredAngleQueue.Average(), rotateLerp));
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
            float desiredAngle = Vector2.SignedAngle(Vector2.up, moveVector);
            desiredAngleQueue.Enqueue(desiredAngle);
        }
    }
}