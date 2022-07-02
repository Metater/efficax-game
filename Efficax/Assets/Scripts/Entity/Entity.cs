using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private double sweepTimeDelay;

    private bool firstUpdateReceived = false;
    private uint leadingTick = 0;

    private uint timePivotTick = 0;
    private double timePivot = 0;

    private (Vector2 pos, uint tickId)[] interpolationBuffer;

    public void Init()
    {

    }

    private void Awake()
    {
        interpolationBuffer = new (Vector2 pos, uint tickId)[256];
        for (int i = 0; i < 256; i++)
        {
            interpolationBuffer[i].tickId = uint.MaxValue;
        }
    }

    private void Start()
    {
        print("created entity");
    }

    private void Update()
    {
        (Vector2 pos, uint tickId) pastUpdate = (Vector2.zero, uint.MaxValue);
        (Vector2 pos, uint tickId) futureUpdate = (Vector2.zero, uint.MaxValue);

        double actualAndPivotTimeDelta = Time.timeAsDouble - timePivot;
        double sweepTime = TickToSeconds(timePivotTick) + actualAndPivotTimeDelta;
        print(sweepTime);
        //sweepTime = TickToSeconds(leadingTick);

        for (int i = 0; i < 256; i++)
        {
            var update = interpolationBuffer[i];

            // Check if update is null
            if (update.tickId == uint.MaxValue)
                continue;

            double updateTime = TickToSeconds(update.tickId);
            double updateAndSweepDeltaTime = updateTime - sweepTime;

            if (updateAndSweepDeltaTime < 0) // past update
            {
                if (updateAndSweepDeltaTime > TickToSeconds(pastUpdate.tickId) - sweepTime || pastUpdate.tickId == uint.MaxValue)
                    pastUpdate = update;
            }
            else // future update
            {
                if (updateAndSweepDeltaTime < TickToSeconds(futureUpdate.tickId) - sweepTime || futureUpdate.tickId == uint.MaxValue)
                    futureUpdate = update;
            }
        }

        if (pastUpdate.tickId != uint.MaxValue && futureUpdate.tickId != uint.MaxValue)
        {
            double step = InverseLerpDouble(TickToSeconds(pastUpdate.tickId), TickToSeconds(futureUpdate.tickId), sweepTime);
            Vector2 pos = Vector2.Lerp(pastUpdate.pos, futureUpdate.pos, (float)step);
            transform.position = pos;
        }
        else
        {
            print("NO LERP DATA");
        }
    }

    public virtual void UpdateEnity(EntitySnapshotData data)
    {
        print(data.TickId);

        if (!firstUpdateReceived)
        {
            firstUpdateReceived = true;
            leadingTick = data.TickId;
            transform.position = data.Pos;
            timePivotTick = data.TickId;
            timePivot = Time.timeAsDouble;
            print(timePivot);
        }
        else
        {
            if (data.TickId > leadingTick)
            {
                leadingTick = data.TickId;
            }
        }

        interpolationBuffer[data.TickId % 256] = (data.Pos, data.TickId);
    }

    private static double TickToSeconds(uint tick)
    {
        return tick * 0.04;
    }

    private static double InverseLerpDouble(double t, double a, double b)
    {
        return (t - a) / (b - a);
    }
}