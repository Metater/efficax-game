using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
using NetCoreServer;
using LiteNetLib;

public class RustLNLServer : TcpServer
{
    public RustLNLServer(IPAddress address, int port) : base(address, port) {}

    protected override TcpSession CreateSession() { return new RustLNLSession(this); }

    protected override void OnError(SocketError error)
    {
        Console.WriteLine($"Chat TCP server caught an error with code {error}");
    }
}