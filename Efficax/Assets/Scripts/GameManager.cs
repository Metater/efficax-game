using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GameManager : MonoBehaviour
{
    public WorldManager worldManager;
    public EntityManager entityManager;
    public PacketManager packetManager;

    public NetworkManager networkManager;

    private ulong ticks = 0;

    private byte lastInput = 255;

    //private TcpChatServer.ChatServer test;

    private void Start()
    {
        networkManager = new NetworkManager(packetManager, "127.0.0.1", 8080);
        networkManager.OptionNoDelay = true;
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
            if (lastInput != input)
            {
                lastInput = input;
                networkManager.SendAsync(new byte[] { 0, input });
            }
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
