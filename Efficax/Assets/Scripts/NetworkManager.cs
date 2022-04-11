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
        bool send = SendAsync(new byte[] { 0, 0, 1, 1, 2, 2, 3, 3 });
        Debug.Log("Sent data: " + send);
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
