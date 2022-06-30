using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlayerManager : MonoBehaviour
{
    [SerializeField] private float cameraFollowSmoothTime;
    [SerializeField] private float cameraFollowMaxSpeed;

    public bool IsPlayerIdSet { get; private set; } = false;
    public uint PlayerId { get; private set; }

    private Vector2 cameraFollowVelocity = Vector2.zero;

    private void Update()
    {
        if (GameManager.I.IsDisconnected)
        {
            IsPlayerIdSet = false;
        }
    }

    private void LateUpdate()
    {
        // TryGet bc there may be gap between join packet setting player id, and the player being spawned in
        if (IsPlayerIdSet && GameManager.I.entityManager.TryGetEntity(PlayerId, out Entity entity))
        {
            Transform player = entity.transform;
            Transform camera = Camera.main.transform;
            Vector2 output = Vector2.SmoothDamp(camera.position, player.position, ref cameraFollowVelocity, cameraFollowSmoothTime, cameraFollowMaxSpeed);
            camera.position = new Vector3(output.x, output.y, camera.position.z);
        }
    }

    public void SetPlayerId(uint playerId)
    {
        if (!IsPlayerIdSet)
        {
            IsPlayerIdSet = true;
            PlayerId = playerId;
        }
    }
}
