using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public struct MCollider
{
    public uint Id { get; private set; }
    public Vector2 Min { get; private set; }
    public Vector2 Max { get; private set; }

    public MCollider(Vector2 min, Vector2 max)
    {
        Id = 0;
        Min = min;
        Max = max;
    }

    public MCollider(uint id, Vector2 min, Vector2 max)
    {
        Id = id;
        Min = min;
        Max = max;
    }

    public static MCollider None()
    {
        return new MCollider(Vector2.zero, Vector2.zero);
    }

    public static MCollider All()
    {
        return new MCollider(Vector2.negativeInfinity, Vector2.positiveInfinity);
    }

    public bool IsStatic()
    {
        return Id != 0;
    }

    public MCollider CopyWithId(uint id)
    {
        return new MCollider(id, Min, Max);
    }

    public MCollider CopyWithOffset(Vector2 offset)
    {
        return new MCollider(Id, Min + offset, Max + offset);
    }

    public bool Intersects(MCollider other)
    {
        if (Max.x < other.Min.x || Min.x > other.Max.x)
        {
            return false;
        }
        if (Max.y < other.Min.y || Min.y > other.Max.y)
        {
            return false;
        }
        return true;
    }
}
