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
        packetManager.ExecuteActions();
    }

    private void FixedUpdate()
    {
        if (!networkManager.IsConnected)
            return;
        //networkManager.ReceiveAsync();
        //test.Multicast(new byte[] { 2, 0, 0, 0, 0, 0, 0, 0, 0 });
        if (ticks % 2 == 0)
        {
            networkManager.SendAsync(new byte[] { 0, GetInput() });
        }
        ticks++;
    }

    private byte GetInput()
    {
        bool w = Input.GetKey(KeyCode.W);
        bool s = Input.GetKey(KeyCode.S);
        bool a = Input.GetKey(KeyCode.A);
        bool d = Input.GetKey(KeyCode.D);
        if (!w && !s && !a && !d) return 0;
        if (w && !s && !a && !d) return 1;
        if (w && !s && !a && d) return 2;
        if (!w && !s && !a && d) return 3;
        if (!w && s && !a && d) return 4;
        if (!w && s && !a && !d) return 5;
        if (!w && s && a && !d) return 6;
        if (!w && !s && a && !d) return 7;
        if (w && !s && a && !d) return 8;
        return 0;
    }

    private void OnApplicationQuit()
    {
        networkManager.Disconnect();
    }
}
