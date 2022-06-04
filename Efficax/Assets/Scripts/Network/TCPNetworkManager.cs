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

public class TCPNetworkManager : TcpClient
{
    private readonly PacketManager packetManager;

    private readonly Deque<byte> ringBuffer = new();
    private readonly NetDataReader reader = new();

    private bool stop = false;

    public TCPNetworkManager(PacketManager packetManager, string address, int port) : base(address, port)
    {
        OptionNoDelay = true;

        this.packetManager = packetManager;
    }

    public void DisconnectAndStop()
    {
        stop = true;
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

        if (!stop)
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

        if (ringBuffer.Count >= 3)
        {
            int dataRead = 0;
            byte[] ringBufferData = ringBuffer.ToArray();

            while (ringBuffer.Count >= 3)
            {
                ushort packetSize = BitConverter.ToUInt16(new byte[] { ringBuffer.RemoveFromFront(), ringBuffer.RemoveFromFront() });
                dataRead += 2;
                byte tickId = ringBuffer.RemoveFromFront();
                dataRead += 1;

                reader.SetSource(ringBufferData, dataRead, dataRead + packetSize);
                while (reader.AvailableBytes > 0)
                {
                    packetManager.Handle(reader, true, tickId);
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
}
