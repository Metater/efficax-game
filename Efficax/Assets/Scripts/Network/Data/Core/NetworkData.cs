using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public abstract class NetworkData<T> where T : class
{
    public uint TickId { get; protected set; }

    public T SetTickIdAndRead(NetDataReader reader, uint tickId)
    {
        TickId = tickId;
        return Read(reader);
    }

    public virtual T Read(NetDataReader reader)
    {
        return this as T;
    }

    public virtual T Write(NetDataReader reader)
    {
        return this as T;
    }
}
