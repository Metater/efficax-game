using System;
using System.Threading;
using System.Net;
using System.Net.Sockets;
using System.Text;
using NetCoreServer;

public class RustLNLSession : TcpSession
{
    public bool IsRunning { get; private set; } = false;

    public RustLNLSession(TcpServer server) : base(server) {}

    protected override void OnConnected()
    {
        IsRunning = true;
        Console.WriteLine($"connected: {Id}");

        Thread lnl = new Thread(new ThreadStart(() => {
            
        }));
    }

    protected override void OnDisconnected()
    {
        IsRunning = false;
        Console.WriteLine($"disconnected: {Id}");
    }

    protected override void OnReceived(byte[] buffer, long offset, long size)
    {

    }

    protected override void OnError(SocketError error)
    {
        Console.WriteLine($"error: {error}");
    }
}