using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using NetCoreServer;
using System.Threading;
using System;
using System.Text;
using System.Net.Sockets;
using TcpClient = NetCoreServer.TcpClient;
using Nito.Collections;
using System.Net;

public class TCPManager : TcpClient
{
    private readonly PacketManager packetManager;

    private readonly Deque<byte> ringBuffer = new();

    private readonly NetDataReader reader = new();
    private readonly NetDataWriter writer = new();

    public bool IsStopped { get; private set; } = false;

    #region Session Management
    public TCPManager(PacketManager packetManager, string address, int port) : base(address, port)
    {
        OptionNoDelay = true;

        this.packetManager = packetManager;
    }

    public void DisconnectAndStop()
    {
        IsStopped = true;
        DisconnectAsync();
        while (IsConnected)
            Thread.Yield();
    }

    protected override void OnConnected()
    {
        Debug.Log($"TCP client connected a new session with Id {Id}");
    }

    protected override void OnDisconnected()
    {
        Debug.Log($"TCP client disconnected a session with Id {Id}");

        Thread.Sleep(1000);

        if (!IsStopped)
        {
            Debug.Log("TCP client reconnecting...");
            ConnectAsync();
        }
    }

    protected override void OnReceived(byte[] buffer, long offset, long size)
    {
        byte[] data = new byte[size];
        Array.Copy(buffer, offset, data, 0, size);
        ringBuffer.InsertRange(ringBuffer.Count, data);

        if (ringBuffer.Count >= 6)
        {
            int dataRead = 0;
            byte[] ringBufferData = ringBuffer.ToArray();

            while (ringBuffer.Count >= 6)
            {
                ushort packetSize = BitConverter.ToUInt16(new byte[] { ringBuffer.RemoveFromFront(), ringBuffer.RemoveFromFront() });
                dataRead += 2;
                uint tickId = BitConverter.ToUInt32(new byte[] { ringBuffer.RemoveFromFront(), ringBuffer.RemoveFromFront(), ringBuffer.RemoveFromFront(), ringBuffer.RemoveFromFront() });
                dataRead += 4;

                reader.SetSource(ringBufferData, dataRead, dataRead + packetSize);
                while (reader.AvailableBytes > 0)
                {
                    packetManager.HandleTcp(reader, tickId);
                }
                ringBuffer.RemoveRange(0, packetSize);
                dataRead += packetSize;
            }
        }
    }

    protected override void OnError(SocketError error)
    {
        Debug.Log($"TCP client caught an error with code {error}");
    }
    #endregion Session Management

    #region Sending Management
    private void ResetWriter()
    {
        writer.Reset();
        writer.Put(ushort.MinValue);
    }
    private void Send()
    {
        var data = writer.Data;

        var length = data.Length;
        writer.SetPosition(0);
        writer.Put((ushort)(length - 2));

        SendAsync(data, 0, length);
    }

    public void SendInitNetwork(ushort udpPort)
    {
        ResetWriter();

        writer.Put(Network.ClientToServer.Tcp.InitNetwork);
        writer.Put(udpPort);

        Send();
    }
    #endregion Sending Management
}
