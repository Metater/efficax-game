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

    [SerializeField] private float interpolationSweepDelay;

    private Queue<float> desiredAngleQueue;

    private bool init = false;
    private byte leadingTick = 0;

    private GameManager gameManager;

    private (float time, Vector2 pos)[] interpolationBuffer;

    public void Init(GameManager gameManager)
    {
        this.gameManager = gameManager;
    }

    private void Awake()
    {
        desiredAngleQueue = new Queue<float>(new float[] { 0 });
        interpolationBuffer = new (float, Vector2)[256];
    }

    private void Start()
    {
        print("created entity");
    }

    private void Update()
    {
        (float time, Vector2 pos)[] sweepedUpdates = new (float, Vector2)[2];
        float sweepTime = Time.time - interpolationSweepDelay;

        // will cause movement delay when entity spawns in for this client
        for (int i = 0; i < 256; i++)
        {
            int index = leadingTick - i;
            if (index < 0)
                index = 256 + index;

            (float time, Vector2 pos) = interpolationBuffer[index];
            if (Time.time - time < 2f && time != 0)
            {
                if (time <= sweepTime) // past
                {
                    if (Mathf.Abs(time - sweepTime) < Mathf.Abs(sweepedUpdates[0].time - sweepTime) || sweepedUpdates[0].time == 0)
                    {
                        sweepedUpdates[0] = (time, pos);
                    }
                }
                else // future
                {
                    if (Mathf.Abs(time - sweepTime) < Mathf.Abs(sweepedUpdates[1].time - sweepTime) || sweepedUpdates[1].time == 0)
                    {
                        sweepedUpdates[1] = (time, pos);
                    }
                }
            }
        }

        if (sweepedUpdates[0].time != 0 && sweepedUpdates[1].time != 0)
        {
            float step = Mathf.InverseLerp(sweepedUpdates[0].time, sweepedUpdates[1].time, sweepTime);
            sprite.transform.position = Vector2.Lerp(sweepedUpdates[0].pos, sweepedUpdates[1].pos, step);
        }
    }

    private void FixedUpdate()
    {

        // TODO think about out of order packets are treated, how does that wierd time effect lerping? yes
        // TODO Eventually set rb.MovePosition every frame, interpolation for rbs only works between fixed updates
        while (desiredAngleQueue.Count > 2)
        {
            desiredAngleQueue.Dequeue();
        }
        rb.MoveRotation(Mathf.LerpAngle(transform.localEulerAngles.z, desiredAngleQueue.Average(), rotateLerp));
    }

    public virtual void UpdateEnity(EntityUpdateData data, byte tickId)
    {
        //if (UnityEngine.Random.Range(0, 100) < (5f / 25f) * 100f)
           // return;

        if (!init)
        {
            init = true;
            leadingTick = tickId;
            UpdatePosition(data.pos);
        }
        else
        {
            if (tickId > leadingTick)
            {
                leadingTick = tickId;
            }
            else if (leadingTick > 127 && tickId < 63)
            {
                leadingTick = tickId;
            }
        }

        // TODO DONT SAVE UPDATE IF SWEEP COULD HIT IT
        // CALCULATE TIME.TIME LATER
        // WILL NEED TO OFFSET LATER ^^^^ DONT TRUST TWO RATES ON DIFF COMPUTERS?
        interpolationBuffer[tickId] = (Time.time, data.pos);
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