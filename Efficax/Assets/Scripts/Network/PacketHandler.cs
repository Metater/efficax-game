using System;

public class PacketHandler
{
    private readonly Action<NetDataReader, byte> packetHandler;

    private PacketHandler(Action<NetDataReader, byte> packetHandler)
    {
        this.packetHandler = packetHandler;
    }

    public static PacketHandler Create<T>(PacketManager packetManager, PacketHandlerType handlerType, Action<T> handler) where T : NetworkData<T>, new()
    {
        void PacketHandler(NetDataReader reader, byte tickId)
        {
            T data = new T().SetTickIdAndRead(reader, tickId);
            switch (handlerType)
            {
                case PacketHandlerType.Default:
                    handler(data);
                    break;
                case PacketHandlerType.Update:
                    packetManager.UpdateQueue.Enqueue(() => handler(data));
                    break;
                case PacketHandlerType.FixedUpdate:
                    packetManager.FixedUpdateQueue.Enqueue(() => handler(data));
                    break;
            }
        }

        return new PacketHandler(PacketHandler);
    }


    public void Handle(NetDataReader reader, byte tickId)
    {
        packetHandler(reader, tickId);
    }
}

public enum PacketHandlerType
{
    Default,
    Update,
    FixedUpdate
}