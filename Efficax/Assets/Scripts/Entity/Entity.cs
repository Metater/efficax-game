using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private float rotateLerp;

    [SerializeField] private float interpolationSweepDelay;

    private Queue<float> desiredAngleQueue;

    private bool init = false;
    private byte leadingTick = 0;

    private GameManager gameManager;

    private Vector3[] interpolationBuffer;

    public void Init(GameManager gameManager)
    {
        this.gameManager = gameManager;
    }

    private void Awake()
    {
        desiredAngleQueue = new Queue<float>(new float[] { 0 });
        interpolationBuffer = new Vector3[256];
    }

    private void Start()
    {
        print("created entity");
    }

    private void Update()
    {
        //(float time, Vector2 pos)[] sweepedUpdates = new (float, Vector2)[2];
        Vector3 lastUpdate = Vector3.zero;
        Vector3 nextUpdate = Vector3.zero;
        float sweepTime = Time.time - interpolationSweepDelay;

        // will cause movement delay when entity spawns in for this client
        for (int i = 0; i < 256; i++)
        {
            Vector3 update = interpolationBuffer[i];

            // Check if update is null
            if (update.z == 0)
                continue;
            // Check if update is expired
            if (Time.time - update.z > 2f)
                continue;

            float delta = update.z - sweepTime;

            if (delta < 0) // past
            {
                //print("past");
                if (delta < lastUpdate.z - sweepTime || lastUpdate.z == 0)
                    lastUpdate = update;
            }
            else // future
            {
                //print("future");
                if (delta > nextUpdate.z - sweepTime || nextUpdate.z == 0)
                    nextUpdate = update;
            }
        }

        if (lastUpdate.z != 0 && nextUpdate.z != 0)
        {
            float step = Mathf.InverseLerp(lastUpdate.z, nextUpdate.z, sweepTime);
            print(sweepTime + ": " + step + ": ");
            transform.position = Vector2.Lerp(lastUpdate, nextUpdate, step);
        }
        else
        {
            print("BAD!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
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
        //rb.MoveRotation(Mathf.LerpAngle(transform.localEulerAngles.z, desiredAngleQueue.Average(), rotateLerp));
    }

    public virtual void UpdateEnity(EntityUpdateData data)
    {
        //if (UnityEngine.Random.Range(0, 100) < (12f / 25f) * 100f)
        //return;

        //print(data.TickId + ": " + data.pos);

        if (!init)
        {
            init = true;
            leadingTick = data.TickId;
            // TODO UPDATE POS ON INIT
            UpdatePosition(data.pos);
        }
        else
        {
            if (data.TickId > leadingTick)
            {
                leadingTick = data.TickId;
            }
            else if (leadingTick > 127 && data.TickId < 63)
            {
                leadingTick = data.TickId;
            }
        }

        // TODO DONT SAVE UPDATE IF SWEEP COULD HIT IT
        // CALCULATE TIME.TIME LATER
        // WILL NEED TO OFFSET LATER ^^^^ DONT TRUST TWO RATES ON DIFF COMPUTERS?
        interpolationBuffer[data.TickId] = new Vector3(data.pos.x, data.pos.y, Time.time);
    }

    private void UpdatePosition(Vector2 pos)
    {
        //Vector2 lastPos = rb.position;
        //rb.MovePosition(pos);
        /*
        if (pos != lastPos)
        {
            Vector2 moveVector = pos - lastPos;
            float desiredAngle = Vector2.SignedAngle(Vector2.up, moveVector);
            desiredAngleQueue.Enqueue(desiredAngle);
        }
        */
    }
}