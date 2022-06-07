using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public abstract class NetworkData
{
    // Network Data Enum Variants
    public const byte Input = 0;
    public const byte Chat = 1;
    public const byte Snapshot = 2;
    public const byte InitUDP = 3;
    public const byte Join = 4;
    public const byte Spawn = 5;
    public const byte Despawn = 6;

    public byte TickId { get; protected set; }
}
