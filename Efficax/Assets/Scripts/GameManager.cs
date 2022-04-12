using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GameManager : MonoBehaviour
{
    public WorldManager worldManager;
    public EntityManager entityManager;
    public PacketManager packetManager;

    public NetworkManager networkManager;

    private void Start()
    {
        networkManager = new NetworkManager(packetManager, "127.0.0.1", 8080);
        print("Client connecting...");
        if (networkManager.ConnectAsync())
        {
            print("Connected!");
        }
    }

    private void Update()
    {
        
    }

    private void OnApplicationQuit()
    {
        networkManager.Disconnect();
    }
}
