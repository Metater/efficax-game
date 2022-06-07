using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public abstract class Tile
{
    protected TilemapManager manager;

    public Vector3Int Position { get; protected set; }
    public Orientation Orientation { get; protected set; }

    public Vector3 FloatPositon => new(Position.x, Position.y);

    public Tile(TilemapManager manager, Vector3Int position, Orientation orientation)
    {
        this.manager = manager;
        Position = position;
        Orientation = orientation;
    }

    public abstract void Render();

    public abstract void Tick(ulong tickId);

    public abstract void Destroy();

    protected void SetQuad(TilemapQuad quad, TileSprite sprite)
    {
        quad.Set(sprite, new Vector3(Position.x, Position.y), GetDegreesOrientation());
    }

    private float GetDegreesOrientation()
    {
        return Orientation switch
        {
            Orientation.Right => 0f,
            Orientation.Up => 90f,
            Orientation.Left => 180f,
            Orientation.Down => 270f,
            _ => 0f,
        };
    }
}
