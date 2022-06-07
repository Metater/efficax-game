using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TilemapQuad
{
    private readonly TilemapLayerRenderer renderer;

    public int Index { get; private set; }
    public TileSprite TileSprite { get; private set; }
    public Vector3 Position { get; private set; }
    public float Rotation { get; private set; }
    public Vector3 QuadSize { get; private set; }

    public bool MarkedToRetire { get; private set; } = false;

    public TilemapQuad(TilemapLayerRenderer renderer, int index)
    {
        this.renderer = renderer;
        Index = index;
    }

    public void Set(TileSprite tileSprite, Vector3 position, float rotation)
    {
        TileSprite = tileSprite;
        Position = position;
        Rotation = rotation;
        QuadSize = new Vector3(1, 1);
        renderer.RenderTilemapQuad(this);
    }

    public void StartRetire()
    {
        MarkedToRetire = true;
        QuadSize = Vector3.zero;
        renderer.RenderTilemapQuad(this);
    }

    public void FinishRetire()
    {
        MarkedToRetire = false;
    }
}
