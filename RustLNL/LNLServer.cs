using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
using NetCoreServer;
using LiteNetLib;

public class LNLServer : INetEventListener
{
    private NetManager server;

    public LNLServer()
    {
        server = new NetManager(this);
    }

    public void OnPeerConnected(NetPeer peer)
    {
        peer.EndPoint
    }

    public void OnPeerDisconnected(NetPeer peer, DisconnectInfo disconnectInfo)
    {

    }
}