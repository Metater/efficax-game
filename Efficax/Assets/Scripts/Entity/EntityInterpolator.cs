using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class EntityInterpolator : MonoBehaviour
{
    // Unity
    [SerializeField] private float velocityToUpdateRotation = 1.125f;
    [SerializeField] private float rotationAverageWindowTime = GameManager.TickPeriod * 3;

    [SerializeField] private double sweepTimeRate = 0.005;
    [SerializeField] private double sweepTimeDelay;
    [SerializeField] private double sweepTimeDelayTarget;
    [SerializeField] private double sweepTimeOffset;

    // Private state
    private uint leadingTick = 0;
    private uint pivotTimeTick = 0;
    private double pivotTime = 0;
    private bool startedLerping = false;

    private (Vector2 pos, uint tickId)[] interpolationBuffer;
    private Queue<Vector2> rotationAverageWindow;
    private Queue<double> targetSweepTimeDelayQueue;

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

    public void Init(uint tickId, Vector2 pos)
    {
        leadingTick = tickId;
        transform.position = pos;
        pivotTimeTick = tickId;
        pivotTime = Time.timeAsDouble;

        interpolationBuffer[tickId % 256] = (pos, tickId);
    }

    public void FeedPosition(uint tickId, Vector2 pos)
    {
        if (tickId > leadingTick)
        {
            leadingTick = tickId;
        }

        // TODO: Only push update if tickId is present enough, no stale/past data

        double timeSincePivot = Time.timeAsDouble - pivotTime;
        double sweepTime = Utils.TickToSeconds(pivotTimeTick) + timeSincePivot;
        double delta = Utils.TickToSeconds(tickId) - sweepTime;
        sweepTimeDelayTarget = -delta + GameManager.TickPeriodDouble;
        // TODO: impl expo backoff instead of ^^^^^^^^^^^^^^^^^^

        targetSweepTimeDelayQueue.Enqueue(sweepTimeDelayTarget);
        if (targetSweepTimeDelayQueue.Count > 25)
        {
            targetSweepTimeDelayQueue.Dequeue();
        }
        if (targetSweepTimeDelayQueue.Count == 25)
        {
            // TODO: Why abs?
            sweepTimeOffset = Math.Abs(targetSweepTimeDelayQueue.Max() - targetSweepTimeDelayQueue.Min());
        }

        interpolationBuffer[tickId % 256] = (pos, tickId);
    }

    private void Update()
    {
        if (Math.Abs((sweepTimeDelayTarget + sweepTimeOffset) - sweepTimeDelay) > sweepTimeRate * 16)
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

        sweepTimeDelay = Utils.StepTowardsDouble(sweepTimeDelay, sweepTimeDelayTarget + sweepTimeOffset, Time.deltaTime * sweepTimeRate);
    }
}