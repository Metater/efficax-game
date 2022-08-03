public static class TileSprites
{
    public const int AtlasTilePixelSize = 16;
    public const int AtlasPixelSize = 256;
    public const int AtlasTileSize = AtlasPixelSize / AtlasTilePixelSize;

    public static TileSprite Grass = new(0, 5);
    public static TileSprite Dirt = new(1, 5);
    public static TileSprite[] BasicSplitter = Series(0, 6, 4);

    public static TileSprite[] Series(int x, int y, int seriesLength)
    {
        TileSprite[] series = new TileSprite[seriesLength];
        for (int i = 0; i < seriesLength; i++)
        {
            series[i] = new TileSprite(x + i, y);
        }
        return series;
    }
}
