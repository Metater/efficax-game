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

    private ulong ticks = 0;

    private byte offInput = 0;
    //private byte lastSentInput = 255;
    private byte inputSequence = 0;

    private bool sentUDPPort = false;

    private void Awake()
    {

    }

    private void Start()
    {
        print("Client connecting...");

        tcp = new TCPNetworkManager(packetManager, "127.0.0.1", 8080);
        tcp.ConnectAsync();

        udp = new UDPNetworkManager(packetManager, "127.0.0.1", 8080);
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

        if (ticks % 2 == 0)
        {
            byte input = GetInput();

            if (input == 0)
            {
                input = offInput;
            }

            //if (lastSentInput != input)
            {
                //lastSentInput = input;
                //udp.SendAsync(new byte[] { 0, input, inputSequence++ });
                tcp.SendAsync(new byte[] { 3, 0, 0, input, inputSequence++ });
            }
        }
        else
        {
            offInput = GetInput();
        }
        ticks++;
    }

    private byte GetInput()
    {
        Vector2 moveVector = new Vector2(Input.GetAxisRaw("Horizontal"), Input.GetAxisRaw("Vertical"));
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
