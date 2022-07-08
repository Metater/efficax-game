using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class MEntity
{
    public uint Id { get; private set; }
    public Vector2 Pos { get; private set; }
    public Vector2 Vel { get; private set; } = Vector2.zero;

    public uint currentCellIndex;

    public MEntity(uint id, Vector2 pos)
    {
        Id = id;
        Pos = pos;
    }
}
