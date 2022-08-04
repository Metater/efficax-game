using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PlayerManager : MonoBehaviour
{
    // Unity
    [SerializeField] private float cameraFollowSmoothTime;
    [SerializeField] private float cameraFollowMaxSpeed;

    // Public state
    public bool IsPlayerIdSet { get; private set; } = false;
    public uint PlayerId { get; private set; }

    // Private state
    private Vector2 cameraFollowVelocity = Vector2.zero;
    private uint inputTicks = 0;
    private byte oddInputDirection = 0;
    private byte inputSequence = 0;

    private void Awake()
    {
        GameManager.I.OnDisconnected += () =>
        {
            IsPlayerIdSet = false;

            cameraFollowVelocity = Vector2.zero;
            inputTicks = 0;
            oddInputDirection = 0;
            inputSequence = 0;
        };

        GameManager.I.packetManager.AddUdpHandler(Network.ServerToClient.Tcp.Join, PacketHandlerType.Update, (JoinData data) =>
        {
            IsPlayerIdSet = true;
            PlayerId = data.PlayerId;
            GameManager.I.entityManager.Spawn(data.TickId, EntityType.Player, data.PlayerId, data.Pos);
        });
    }

    private void LateUpdate()
    {
        if (IsPlayerIdSet)
        {
            CameraFollow();
        }
    }

    private void FixedUpdate()
    {
        #region Input Handling
        if (inputTicks % 2 == 0)
        {
            byte inputDirection = GetInputDirection();
            if (inputDirection == 0)
            {
                inputDirection = oddInputDirection;
            }
            GameManager.I.udp.SendInput(inputDirection, inputSequence++);
        }
        else
        {
            oddInputDirection = GetInputDirection();
        }
        inputTicks++;
        #endregion Input Handling
    }

    private byte GetInputDirection()
    {
        Vector2 moveVector = new(Input.GetAxisRaw("Horizontal"), Input.GetAxisRaw("Vertical"));
        if (moveVector == Vector2.zero)
            return 0;
        float angle = 0.5f - (Mathf.Atan2(-moveVector.x, -moveVector.y) / (-2 * Mathf.PI));
        return (byte)(Mathf.RoundToInt(angle * 8) + 1);
    }

    private void CameraFollow()
    {
        var entity = GameManager.I.entityManager.Entities[PlayerId];
        Transform player = entity.transform;
        Transform camera = Camera.main.transform;
        Vector2 output = Vector2.SmoothDamp(camera.position, player.position, ref cameraFollowVelocity, cameraFollowSmoothTime, cameraFollowMaxSpeed);
        camera.position = new Vector3(output.x, output.y, camera.position.z);
    }
}
