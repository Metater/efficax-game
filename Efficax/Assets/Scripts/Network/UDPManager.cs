using NetCoreServer;
using System.Collections;
using System.Collections.Generic;
using System.Net;
using System.Net.Sockets;
using System.Threading;
using UnityEngine;
using UdpClient = NetCoreServer.UdpClient;

public class UDPManager : UdpClient
{
    private readonly PacketManager packetManager;

    private readonly NetDataReader reader = new();
    private readonly NetDataWriter writer = new();

    public bool IsStopped { get; private set; } = false;

    #region Session Management
    public UDPManager(PacketManager packetManager, string address, int port) : base(address, port)
    {
        this.packetManager = packetManager;
    }

    public void DisconnectAndStop()
    {
        IsStopped = true;
        Disconnect();
        while (IsConnected)
        {
            Thread.Yield();
        }
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

        if (!IsStopped)
        {
            Debug.Log("UDP client reconnecting...");
            Connect();
        }
    }

    protected override void OnReceived(EndPoint endpoint, byte[] buffer, long offset, long size)
    {
        reader.SetSource(buffer, (int)offset, (int)size);

        uint tickId = reader.GetUInt();

        while (reader.AvailableBytes > 0)
        {
            packetManager.HandleUdp(reader, tickId);
        }

        ReceiveAsync();
    }

    protected override void OnError(SocketError error)
    {
        Debug.Log($"UDP client caught an error with code {error}");
    }

    public ushort GetLocalPort() => (ushort)(Socket.LocalEndPoint as IPEndPoint).Port;
    #endregion Session Management

    #region Sending Management
    private void Send()
    {
        SendAsync(writer.Data, 0, writer.Length);
    }

    public void SendInput(byte inputDirection, byte inputSequence)
    {
        writer.Reset();

        writer.Put(Network.ClientToServer.Udp.Input);
        writer.Put(inputDirection);
        writer.Put(inputSequence);

        Send();
    }
    #endregion Sending Management
}
