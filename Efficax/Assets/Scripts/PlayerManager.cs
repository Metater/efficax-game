using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlayerManager : MonoBehaviour
{
    [SerializeField] private GameManager gameManager;

    [SerializeField] private float cameraFollowSmoothTime;

    public bool IsPlayerIdSet { get; private set; } = false;
    public ulong PlayerId { get; private set; }

    private Vector2 cameraFollowVelocity = Vector2.zero;

    private void Update()
    {
        if (gameManager.IsDisconnected)
        {
            IsPlayerIdSet = false;
        }
    }

    private void LateUpdate()
    {
        if (IsPlayerIdSet)
        {
            Transform player = gameManager.entityManager.GetEntity(PlayerId).transform;
            Transform camera = Camera.main.transform;
            Vector2 output = Vector2.SmoothDamp(camera.position, player.position, ref cameraFollowVelocity, cameraFollowSmoothTime);
            camera.position = new Vector3(output.x, output.y, camera.position.z);
        }
    }

    public void SetPlayerId(ulong playerId)
    {
        if (!IsPlayerIdSet)
        {
            IsPlayerIdSet = true;
            PlayerId = playerId;
        }
    }
}