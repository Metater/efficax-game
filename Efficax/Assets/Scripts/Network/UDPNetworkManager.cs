using NetCoreServer;
using System.Collections;
using System.Collections.Generic;
using System.Net;
using System.Net.Sockets;
using System.Threading;
using UnityEngine;
using UdpClient = NetCoreServer.UdpClient;

public class UDPNetworkManager : UdpClient
{
    private PacketManager packetManager;

    private NetDataReader reader = new NetDataReader();

    private bool stop = false;

    public UDPNetworkManager(PacketManager packetManager, string address, int port) : base(address, port)
    {
        this.packetManager = packetManager;
    }

    public void DisconnectAndStop()
    {
        stop = true;
        Disconnect();
        while (IsConnected)
            Thread.Yield();
    }

    protected override void OnConnected()
    {
        Debug.Log($"UDP client connected a new session with Id {Id}");
        ReceiveAsync();
    }

    protected override void OnDisconnected()
    {
        Debug.Log($"UDP client disconnected a session with Id {Id}");

        Thread.Sleep(1000);

        if (!stop)
        {
            Debug.Log("UDP client reconnecting...");
            Connect();
        }
    }

    protected override void OnReceived(EndPoint endpoint, byte[] buffer, long offset, long size)
    {
        reader.SetSource(buffer, (int)offset, (int)size);

        byte tickId = reader.GetByte();

        while (reader.AvailableBytes > 0)
        {
            packetManager.Handle(reader, false, tickId);
        }

        ReceiveAsync();
    }

    protected override void OnError(SocketError error)
    {
        Debug.Log($"UDP client caught an error with code {error}");
    }
}
