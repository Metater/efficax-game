using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private float velocityToUpdateRotation;
    [SerializeField] private float rotationAverageWindowTime;

    [SerializeField] private double sweepTimeVelocity;
    [SerializeField] private double sweepTimeDelay;
    [SerializeField] private double sweepTimeDelayTarget;
    [SerializeField] private double sweepTimeOffset;

    private bool isInit = false;
    private uint leadingTick = 0;

    private uint pivotTimeTick = 0;
    private double pivotTime = 0;

    private bool startedLerping = false;

    private (Vector2 pos, uint tickId)[] interpolationBuffer;

    private Queue<Vector2> rotationAverageWindow;

    private Queue<double> targetSweepTimeDelayQueue;

    public void Init(Vector2 pos)
    {
        transform.position = pos;
    }

    private void Awake()
    {
        interpolationBuffer = new (Vector2 pos, uint tickId)[256];
        for (int i = 0; i < 256; i++)
        {
            interpolationBuffer[i].tickId = uint.MaxValue;
        }

        rotationAverageWindow = new();
        targetSweepTimeDelayQueue = new();
    }

    private void Start()
    {
        print("created entity");
    }

    private void Update()
    {
        if (Math.Abs((sweepTimeDelayTarget + sweepTimeOffset) - sweepTimeDelay) > sweepTimeVelocity * 16)
        {
            sweepTimeDelay = (sweepTimeDelayTarget + sweepTimeOffset);
        }

        (Vector2 pos, uint tickId) pastUpdate = (Vector2.zero, uint.MaxValue);
        (Vector2 pos, uint tickId) futureUpdate = (Vector2.zero, uint.MaxValue);

        double timeSincePivot = Time.timeAsDouble - pivotTime;
        double sweepTime = (Utils.TickToSeconds(pivotTimeTick) + timeSincePivot) - sweepTimeDelay;

        for (int i = 0; i < 256; i++)
        {
            var update = interpolationBuffer[i];

            // Check if update is null
            if (update.tickId == uint.MaxValue)
                continue;

            double updateTime = Utils.TickToSeconds(update.tickId);
            double updateAndSweepDeltaTime = updateTime - sweepTime;

            if (updateAndSweepDeltaTime < 0) // past update
            {
                if (updateAndSweepDeltaTime > Utils.TickToSeconds(pastUpdate.tickId) - sweepTime || pastUpdate.tickId == uint.MaxValue)
                    pastUpdate = update;
            }
            else // future update
            {
                if (updateAndSweepDeltaTime < Utils.TickToSeconds(futureUpdate.tickId) - sweepTime || futureUpdate.tickId == uint.MaxValue)
                    futureUpdate = update;
            }
        }

        float angle = transform.localEulerAngles.z;

        if (pastUpdate.tickId != uint.MaxValue && futureUpdate.tickId != uint.MaxValue)
        {
            startedLerping = true;
            double pastTime = Utils.TickToSeconds(pastUpdate.tickId);
            double futureTime = Utils.TickToSeconds(futureUpdate.tickId);
            double step = Utils.InverseLerpDouble(sweepTime, pastTime, futureTime);
            Vector2 lastPos = transform.position;
            Vector2 pos = Vector2.LerpUnclamped(pastUpdate.pos, futureUpdate.pos, (float)step);
            transform.position = pos;
            if (lastPos != pos && (Vector2.Distance(pos, lastPos) / Time.deltaTime > velocityToUpdateRotation))
            {
                angle = Vector2.SignedAngle(Vector2.up, pos - lastPos);
            }
        }
        else
        {
            if (startedLerping)
            {
                print("NO LERP DATA");
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

        sweepTimeDelay = Utils.StepTowardsDouble(sweepTimeDelay, sweepTimeDelayTarget + sweepTimeOffset, Time.deltaTime * sweepTimeVelocity);
    }

    public virtual void Snapshot(EntitySnapshotData data)
    {
        //if (UnityEngine.Random.Range(0f, 100f) < (5f / 25f) * 100f)
        //return;

        RawSnapshot(data.TickId, data.Pos);
    }

    public void RawSnapshot(uint tickId, Vector2 pos)
    {
        if (!isInit)
        {
            isInit = true;
            leadingTick = tickId;
            transform.position = pos;
            pivotTimeTick = tickId;
            pivotTime = Time.timeAsDouble;
        }
        else
        {
            if (tickId > leadingTick)
            {
                leadingTick = tickId;
            }

            double timeSincePivot = Time.timeAsDouble - pivotTime;
            double sweepTime = Utils.TickToSeconds(pivotTimeTick) + timeSincePivot;
            double delta = Utils.TickToSeconds(tickId) - sweepTime;
            sweepTimeDelayTarget = -delta + 0.04;
            // TODO: impl expo backoff instead of ^^^^^^^^^^^^^^^^^^

            targetSweepTimeDelayQueue.Enqueue(sweepTimeDelayTarget);
            if (targetSweepTimeDelayQueue.Count > 25)
            {
                targetSweepTimeDelayQueue.Dequeue();
            }
            if (targetSweepTimeDelayQueue.Count == 25)
            {
                sweepTimeOffset = Math.Abs(targetSweepTimeDelayQueue.Max() - targetSweepTimeDelayQueue.Min());
            }
        }

        interpolationBuffer[tickId % 256] = (pos, tickId);
    }
}