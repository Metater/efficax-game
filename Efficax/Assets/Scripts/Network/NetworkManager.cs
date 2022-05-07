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

public class NetworkManager : TcpClient
{
    private PacketManager packetManager;

    private Deque<byte> ringBuffer = new Deque<byte>();
    private NetDataReader reader = new NetDataReader();

    private bool tryReconnect = true;

    public NetworkManager(PacketManager packetManager, string address, int port) : base(address, port)
    {
        OptionNoDelay = true;

        this.packetManager = packetManager;
    }

    public void Disconnect(bool tryReconnect)
    {
        this.tryReconnect = tryReconnect;
        Disconnect();
    }

    protected override void OnConnected()
    {
        Debug.Log($"TCP client connected a new session with Id {Id}");

        /*
        for (int i = 0; i < 100; i++)
        {
            SendAsync(new byte[] { 0, 255 });
        }
        */
        //SendAsync(new byte[] { 3 });

        //bool s = SendAsync(new byte[] { 1, 48, 69, 20, 64, 61, 64 });
        //Debug.Log("Sent data: " + s);

        /*
        byte[] b = Encoding.UTF8.GetBytes("Hello, world!");
        byte[] buf = new byte[b.Length + 1];
        buf[0] = 1;
        b.CopyTo(buf, 1);
        bool se = SendAsync(buf);
        Debug.Log("Sent data: " + se);
        */
    }

    protected override void OnDisconnected()
    {
        Debug.Log($"TCP client disconnected a session with Id {Id}");

        if (!tryReconnect)
            return;

        Thread.Sleep(1000);
        ConnectAsync();
    }

    protected override void OnReceived(byte[] buffer, long offset, long size)
    {
        byte[] data = new byte[size];
        Array.Copy(buffer, offset, data, 0, size);
        ringBuffer.InsertRange(ringBuffer.Count, data);

        if (ringBuffer.Count >= 2)
        {
            int dataRead = 0;
            byte[] ringBufferData = ringBuffer.ToArray();

            while (ringBuffer.Count >= 2)
            {
                ushort packetSize = BitConverter.ToUInt16(new byte[] { ringBuffer.RemoveFromFront(), ringBuffer.RemoveFromFront() });
                dataRead += 2;
                reader.SetSource(ringBufferData, dataRead, packetSize);
                ringBuffer.RemoveRange(0, packetSize);
                dataRead += packetSize;
            }
        }

        while (reader.AvailableBytes > 0)
        {
            packetManager.Handle(reader);
        }
    }

    protected override void OnError(SocketError error)
    {
        Debug.Log($"TCP client caught an error with code {error}");
    }
}
