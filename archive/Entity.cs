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

    [SerializeField] private float lerpSmoothTime;

    [SerializeField] private Transform lookAhead;

    private bool isInit = false;
    private uint leadingTick = 0;

    private Queue<Vector2> rotationAverageWindow;

    private Vector2 shadowPos = Vector2.zero;

    public void Init(Vector2 pos)
    {
        transform.position = pos;
        shadowPos = pos;
    }

    private void Awake()
    {
        rotationAverageWindow = new();
    }

    private void Start()
    {
        print("created entity");
    }

    private Vector2 v;

    private void Update()
    {
        //transform.position = Vector2.Lerp(lastPos, shadowPos, (Time.timeAsDouble - posTime) / )
        transform.position = Vector2.SmoothDamp(transform.position, shadowPos, ref v, lerpSmoothTime);
        /*
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
        */
    }

    public virtual void Snapshot(EntitySnapshotData data)
    {
        if (UnityEngine.Random.Range(0f, 100f) < (1f / 25f) * 100f)
            return;

        RawSnapshot(data.TickId, data.Pos);
    }

    public void RawSnapshot(uint tickId, Vector2 pos)
    {
        if (!isInit)
        {
            isInit = true;
        }
        else
        {
            if (tickId > leadingTick)
            {
                leadingTick = tickId;
                shadowPos = pos;
                lookAhead.transform.position = shadowPos;
            }
        }
    }
}