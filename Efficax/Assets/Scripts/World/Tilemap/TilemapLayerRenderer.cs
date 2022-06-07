using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TilemapLayerRenderer : MonoBehaviour
{
    public const int QuadArraySize = 16383;

    private Mesh mesh;

    private Vector3[] vertices;
    private Vector2[] uv;
    private int[] triangles;

    private readonly List<TilemapQuad> quads = new();
    private readonly Queue<TilemapQuad> retiredQuads = new();
    private readonly Queue<TilemapQuad> renderQueue = new();
    private int nextQuadIndex = 0;

    private void Awake()
    {
        mesh = new Mesh();
        GetComponent<MeshFilter>().mesh = mesh;
        MeshUtils.CreateEmptyMeshArrays(QuadArraySize, out vertices, out uv, out triangles);
    }

    public TilemapQuad GetNextQuad()
    {
        if (retiredQuads.Count > 0)
        {
            return retiredQuads.Dequeue();
        }

        if (nextQuadIndex < QuadArraySize)
        {
            TilemapQuad quad = new(this, nextQuadIndex++);
            quads.Add(quad);
            return quad;
        }

        throw new Exception($"Cannot allocate more quads from mesh!");
    }

    public void RenderTilemapQuad(TilemapQuad quad)
    {
        renderQueue.Enqueue(quad);
    }

    public void Render()
    {
        if (renderQueue.Count == 0)
        {
            return;
        }

        while (renderQueue.Count > 0)
        {
            TilemapQuad quad = renderQueue.Dequeue();
            MeshUtils.AddToMeshArrays(vertices, uv, triangles, quad.Index, quad.Position, quad.Rotation, quad.QuadSize, quad.TileSprite.UV00, quad.TileSprite.UV11);
            if (quad.MarkedToRetire)
            {
                quad.FinishRetire();
                retiredQuads.Enqueue(quad);
            }
        }

        mesh.vertices = vertices;
        mesh.uv = uv;
        mesh.triangles = triangles;
    }
}
