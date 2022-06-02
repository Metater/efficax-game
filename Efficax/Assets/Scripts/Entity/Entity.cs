using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [SerializeField] private float velocityToUpdateRotation;

    [SerializeField] private float interpolationSweepDelay;
    [SerializeField] private float interpolationSweepDelayGrowth;
    [SerializeField] private float interpolationSweepDelayDecay;

    private bool init = false;
    private byte leadingTick = 0;

    private GameManager gameManager;

    private bool lerpStart = false;
    private Vector3[] interpolationBuffer;

    public void Init(GameManager gameManager)
    {
        this.gameManager = gameManager;
    }

    private void Awake()
    {
        interpolationBuffer = new Vector3[256];
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

        // will cause movement delay when entity spawns in for this client
        for (int i = 0; i < 256; i++)
        {
            Vector3 update = interpolationBuffer[i];

            // Check if update is null or expired
            if (update.z == 0 || Time.time - update.z > 2f)
                continue;

            float delta = update.z - sweepTime;

            if (delta < 0) // past
            {
                //print("past");
                if (delta > lastUpdate.z - sweepTime || lastUpdate.z == 0)
                    lastUpdate = update;
            }
            else // future
            {
                //print("future");
                if (delta < nextUpdate.z - sweepTime || nextUpdate.z == 0)
                    nextUpdate = update;
            }
        }

        if (lastUpdate.z != 0 && nextUpdate.z != 0)
        {
            lerpStart = true;
            float step = Mathf.InverseLerp(lastUpdate.z, nextUpdate.z, sweepTime);
            Vector2 lastPos = transform.position;
            Vector2 pos = Vector2.Lerp(lastUpdate, nextUpdate, step);
            transform.position = pos;
            if (lastPos != pos && Vector2.Distance(pos, lastPos) / Time.deltaTime > velocityToUpdateRotation)
            {
                float angle = Vector2.SignedAngle(Vector2.up, pos - lastPos);
                transform.localEulerAngles = new Vector3(transform.localEulerAngles.x, transform.localEulerAngles.y, angle);
            }
        }
        else
        {
            if (lerpStart)
            {
                print("Insufficient data to lerp!");
                interpolationSweepDelayDecay = 1 - ((1 - interpolationSweepDelayDecay) / 4);
                interpolationSweepDelay *= interpolationSweepDelayGrowth;
            }
        }
    }

    private void FixedUpdate()
    {
        interpolationSweepDelay *= interpolationSweepDelayDecay;
        // TODO think about out of order packets are treated, how does that wierd time effect lerping? yes
        // TODO Eventually set rb.MovePosition every frame, interpolation for rbs only works between fixed updates
    }

    public virtual void UpdateEnity(EntityUpdateData data)
    {
        //if (UnityEngine.Random.Range(0, 100) < (2f / 25f) * 100f)
            //return;

        if (!init)
        {
            init = true;
            leadingTick = data.TickId;
            transform.position = data.pos;
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
}