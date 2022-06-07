using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class StaticTile : Tile
{
    private readonly TileSprite sprite;
    private readonly TilemapQuad quad;

    public StaticTile(TilemapManager manager, Vector3Int position, Orientation orientation, TileSprite sprite) : base(manager, position, orientation)
    {
        this.sprite = sprite;
        quad = manager.GetRenderer(0).GetNextQuad();
    }

    public override void Render()
    {
        SetQuad(quad, sprite);
    }

    public override void Tick(ulong tickId)
    {

    }

    public override void Destroy()
    {
        quad.StartRetire();
    }
}
