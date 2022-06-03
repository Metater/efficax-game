using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using UnityEngine;

public class GameManager : MonoBehaviour
{
    public WorldManager worldManager;
    public EntityManager entityManager;
    public PacketManager packetManager;

    public TCPNetworkManager tcp;
    public UDPNetworkManager udp;

    public ulong ClientTick { get; private set; } = 0;

    private byte offInput = 0;
    private byte inputSequence = 0;

    private bool sentUDPPort = false;

    [SerializeField] private GameObject test;

    private void Awake()
    {

    }

    private void Start()
    {
        print("Client connecting...");

        tcp = new TCPNetworkManager(packetManager, "75.0.193.55", 25569);
        tcp.ConnectAsync();

        udp = new UDPNetworkManager(packetManager, "75.0.193.55", 25569);
        udp.Connect();
    }

    private void Update()
    {
        packetManager.ExecuteQueuedUpdates();
    }

    private void FixedUpdate()
    {
        packetManager.ExecuteQueuedFixedUpdates();

        if (!tcp.IsConnected || !udp.IsConnected)
            return;

        if (!sentUDPPort)
        {
            sentUDPPort = true;
            ushort port = (ushort)(udp.Socket.LocalEndPoint as IPEndPoint).Port;
            tcp.SendAsync(new byte[] { 3, 0, 3, (byte)port, (byte)(port >> 8) });
        }

        if (ClientTick % 2 == 0)
        {
            byte input = GetInput();

            if (input == 0)
            {
                input = offInput;
            }

            udp.SendAsync(new byte[] { 0, input, inputSequence++ });
        }
        else
        {
            offInput = GetInput();
        }
        ClientTick++;
    }

    private byte GetInput()
    {
        Vector2 moveVector = new(Input.GetAxisRaw("Horizontal"), Input.GetAxisRaw("Vertical"));
        if (moveVector == Vector2.zero)
            return 0;
        float angle = 0.5f - (Mathf.Atan2(-moveVector.x, -moveVector.y) / (-2 * Mathf.PI));
        return (byte)(Mathf.RoundToInt(angle * 8) + 1);
    }

    private void OnApplicationQuit()
    {
        tcp.DisconnectAndStop();
        udp.DisconnectAndStop();
    }
}
