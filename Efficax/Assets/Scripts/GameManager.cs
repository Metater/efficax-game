using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using UnityEngine;

public class GameManager : MonoBehaviour
{
    #region Singleton
    private static GameManager instance;
    public static GameManager I { get { return instance; } }
    private void AwakeSingleton()
    {
        if (instance == null)
        {
            instance = this;
        }
        else
        {
            Destroy(gameObject);
            return;
        }
        //DontDestroyOnLoad(gameObject);
    }
    #endregion Singleton

    // Unity managers
    public WorldManager worldManager;
    public EntityManager entityManager;
    public PlayerManager playerManager;

    // Managers
    public PacketManager packetManager;
    public TCPManager tcp;
    public UDPManager udp;

    // Public state
    public bool IsDisconnected => !tcp.IsConnected || !udp.IsConnected;
    public bool HasInitNetwork { get; private set; } = false;

    // Private state
    private bool wasDisconnected = true;

    // Events
    public event Action OnConnected;
    public event Action OnDisconnected;

    private void Awake()
    {
        AwakeSingleton();

        OnDisconnected += () =>
        {
            HasInitNetwork = false;
        };
    }

    private void Start()
    {
        print("Client connecting...");

        packetManager = new();

        tcp = new(packetManager, "127.0.0.1", 8080);
        tcp.ConnectAsync();

        udp = new(packetManager, "127.0.0.1", 8080);
        udp.Connect();
    }

    private void Update()
    {
        packetManager.ExecuteUpdates();

        #region Connection Events
        if (IsDisconnected && !wasDisconnected)
        {
            OnDisconnected?.Invoke();
        }
        else if (!IsDisconnected && wasDisconnected)
        {
            OnConnected?.Invoke();
        }

        wasDisconnected = IsDisconnected;
        #endregion Connection Events
    }

    private void FixedUpdate()
    {
        packetManager.ExecuteFixedUpdates();

        if (IsDisconnected)
        {
            return;
        }

        if (!HasInitNetwork)
        {
            HasInitNetwork = true;
            tcp.SendInitNetwork(udp.GetLocalPort());
        }
    }

    private void OnApplicationQuit()
    {
        tcp.DisconnectAndStop();
        udp.DisconnectAndStop();
    }
}
