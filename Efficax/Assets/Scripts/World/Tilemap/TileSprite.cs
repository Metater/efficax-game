using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TileSprite
{
    public Vector2 UV00 { get; private set; }
    public Vector2 UV11 { get; private set; }

    public TileSprite(int x, int y)
    {
        UV00 = new Vector2((x * TileSprites.AtlasTilePixelSize) / (float)TileSprites.AtlasPixelSize, (y * TileSprites.AtlasTilePixelSize) / (float)TileSprites.AtlasPixelSize);
        UV11 = new Vector2(((x * TileSprites.AtlasTilePixelSize) + TileSprites.AtlasTilePixelSize) / (float)TileSprites.AtlasPixelSize, ((y * TileSprites.AtlasTilePixelSize) + TileSprites.AtlasTilePixelSize) / (float)TileSprites.AtlasPixelSize);
    }
}
