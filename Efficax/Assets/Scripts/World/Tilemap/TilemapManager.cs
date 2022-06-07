using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TilemapManager : MonoBehaviour
{
    [SerializeField] private List<TilemapLayerRenderer> renderers;

    private readonly Tile[,,] tilemap = new Tile[64, 32, 2];

    private void Start()
    {
        for (int z = 0; z < 1; z++)
        {
            for (int y = 0; y < tilemap.GetLength(1); y++)
            {
                for (int x = 0; x < tilemap.GetLength(0); x++)
                {
                    int index = x + y;
                    TileSprite sprite = index % 2 == 0 ? TileSprites.Dirt : TileSprites.Dirt;
                    StaticTile tile = new(this, new Vector3Int(x, y, z), Orientation.Up, sprite);
                    Set(tile);
                }
            }
        }
    }

    private void LateUpdate()
    {
        renderers.ForEach(r => r.Render());
    }

    public void Set(Tile tile)
    {
        Remove(tile.Position);
        tilemap[tile.Position.x, tile.Position.y, tile.Position.z] = tile;
        tile.Render();
    }

    public void Remove(Tile tile)
    {
        Remove(tile.Position);
    }

    public void Remove(Vector3Int position)
    {
        tilemap[position.x, position.y, position.z]?.Destroy();
        tilemap[position.x, position.y, position.z] = null;
    }

    public TilemapLayerRenderer GetRenderer(int layer)
    {
        return renderers[layer];
    }
}
