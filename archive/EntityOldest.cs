using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityOldest : MonoBehaviour
{
    [SerializeField] private float velocityToUpdateRotation;

    [SerializeField] private float interpolationSweepDelay;
    [SerializeField] private float interpolationSweepDelayGrowth;
    [SerializeField] private float interpolationSweepDelayDecay;

    [SerializeField] private float rotationAverageWindowTime;

    private bool firstUpdateReceived = false;
    private uint leadingTick = 0;

    private bool lerpStart = false;
    private Vector3[] interpolationBuffer;
    private Queue<Vector2> rotationAverageWindow;

    public void Init()
    {

    }

    private void Awake()
    {
        interpolationBuffer = new Vector3[256];

        rotationAverageWindow = new();
    }

    private void Start()
    {
        print("created entity");
    }

    private void Update()
    {
        Vector3 lastUpdate = Vector3.zero;
        Vector3 nextUpdate = Vector3.zero;
        float sweepTime = Time.time - interpolationSweepDelay;

        for (int i = 0; i < 256; i++)
        {
            Vector3 update = interpolationBuffer[i];

            // Check if update is null or expired
            if (update.z == 0 || Time.time - update.z > 2f)
                continue;

            float delta = update.z - sweepTime;

            if (delta < 0) // past
            {
                if (delta > lastUpdate.z - sweepTime || lastUpdate.z == 0)
                    lastUpdate = update;
            }
            else // future
            {
                if (delta < nextUpdate.z - sweepTime || nextUpdate.z == 0)
                    nextUpdate = update;
            }
        }

        float angle = transform.localEulerAngles.z;
        if (lastUpdate.z != 0 && nextUpdate.z != 0)
        {
            //float d = nextUpdate.z - lastUpdate.z;
            //print(d);
            lerpStart = true;
            float step = Mathf.InverseLerp(lastUpdate.z, nextUpdate.z, sweepTime);
            Vector2 lastPos = transform.position;
            Vector2 pos = Vector2.Lerp(lastUpdate, nextUpdate, step);
            transform.position = pos;
            if (lastPos != pos && (Vector2.Distance(pos, lastPos) / Time.deltaTime > velocityToUpdateRotation))
            {
                angle = Vector2.SignedAngle(Vector2.up, pos - lastPos);
            }
            //print(((pos - lastPos).x) / Time.deltaTime);
        }
        else
        {
            if (lerpStart)
            {
                //print("Insufficient data to lerp, backing off!");
                interpolationSweepDelayDecay = 1 - ((1 - interpolationSweepDelayDecay) / 4);
                interpolationSweepDelay *= interpolationSweepDelayGrowth;
            }
        }

        rotationAverageWindow.Enqueue(new Vector2(angle, Time.time));
        while (rotationAverageWindow.Count > 0 && rotationAverageWindow.First().y < Time.time - rotationAverageWindowTime)
        {
            rotationAverageWindow.Dequeue();
        }
        if (rotationAverageWindow.Count > 0)
        {
            Vector2 sum = Vector2.zero;
            foreach (var rotation in rotationAverageWindow)
            {
                float angleRad = (rotation.x + 90f) * Mathf.Deg2Rad;
                sum += new Vector2(Mathf.Cos(angleRad), Mathf.Sin(angleRad));
            }
            sum /= rotationAverageWindow.Count;
            transform.localEulerAngles = new Vector3(transform.localEulerAngles.x, transform.localEulerAngles.y, Mathf.Atan2(sum.x, sum.y) * -Mathf.Rad2Deg);
        }
    }

    private void FixedUpdate()
    {
        interpolationSweepDelay *= interpolationSweepDelayDecay;
        if (interpolationSweepDelayDecay > 0.99999f)
        {
            interpolationSweepDelayDecay = 0.999f;
        }
        // TODO think about out of order packets are treated, how does that wierd time effect lerping? yes
    }

    public virtual void UpdateEnity(EntitySnapshotData data)
    {
        //if (UnityEngine.Random.Range(0, 100) < (1f / 25f) * 100f)
        //return;

        if (!firstUpdateReceived)
        {
            firstUpdateReceived = true;
            leadingTick = data.TickId;
            transform.position = data.Pos;
        }
        else
        {
            if (data.TickId > leadingTick)
            {
                leadingTick = data.TickId;
            }
        }

        // TODO DONT SAVE UPDATE IF SWEEP COULD HIT IT
        // CALCULATE TIME.TIME LATER
        // WILL NEED TO OFFSET LATER ^^^^ DONT TRUST TWO RATES ON DIFF COMPUTERS?
        interpolationBuffer[data.TickId % 256] = new Vector3(data.Pos.x, data.Pos.y, Time.time);
    }
}