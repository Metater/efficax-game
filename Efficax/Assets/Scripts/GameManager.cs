using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class GameManager : MonoBehaviour
{
    public WorldManager worldManager;
    public EntityManager entityManager;
    public PacketManager packetManager;

    public NetworkManager networkManager;

    private ulong ticks = 0;

    private byte offInput = 0;
    private byte lastSentInput = 255;
    private byte inputSequence = 0;

    //private TcpChatServer.ChatServer test;

    private void Awake()
    {

    }

    private void Start()
    {
        networkManager = new NetworkManager(packetManager, "127.0.0.1", 8080);
        //networkManager = new NetworkManager(packetManager, "192.168.0.209", 8080);
        print("Client connecting...");
        if (networkManager.ConnectAsync())
        {
            print("Connected!");
        }

        //test = TcpChatServer.Program.Test();
    }

    private void Update()
    {
        packetManager.ExecuteQueuedUpdates();
    }

    private void FixedUpdate()
    {
        packetManager.ExecuteQueuedFixedUpdates();

        if (!networkManager.IsConnected)
            return;
        //networkManager.ReceiveAsync();
        //test.Multicast(new byte[] { 2, 0, 0, 0, 0, 0, 0, 0, 0 });
        if (ticks % 2 == 0)
        {
            byte input = GetInput();

            if (input == 0)
            {
                input = offInput;
            }

            if (lastSentInput != input)
            {
                lastSentInput = input;
                networkManager.SendAsync(new byte[] { 3, 0, 0, input, inputSequence++ });
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
        networkManager.Disconnect();
    }
}
