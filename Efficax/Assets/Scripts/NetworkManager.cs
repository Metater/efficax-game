using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using NetCoreServer;
using System.Threading;
using System;
using System.Text;
using System.Net.Sockets;
using TcpClient = NetCoreServer.TcpClient;

public class NetworkManager : TcpClient
{
    public NetworkManager(string address, int port) : base(address, port) { }

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

        byte[] b = Encoding.UTF8.GetBytes("HelHello, worldHello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!lo, world!");
        byte[] buf = new byte[b.Length + 1];
        buf[0] = 1;
        b.CopyTo(buf, 1);
        bool se = SendAsync(buf);
        Debug.Log("Sent data: " + se);
    }

    protected override void OnDisconnected()
    {
        Debug.Log($"TCP client disconnected a session with Id {Id}");

        /*
        // Wait for a while...
        Thread.Sleep(1000);

        // Try to connect again
        if (!_stop)
            ConnectAsync();
        */
    }

    protected override void OnReceived(byte[] buffer, long offset, long size)
    {
        Debug.Log(Encoding.UTF8.GetString(buffer, (int)offset, (int)size));
    }

    protected override void OnError(SocketError error)
    {
        Debug.Log($"TCP client caught an error with code {error}");
    }
}
