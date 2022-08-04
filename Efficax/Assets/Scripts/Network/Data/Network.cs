public static class Network
{
    public static class ServerToClient
    {
        public static class Tcp
        {
            public const byte Chat = 0;
            public const byte Join = 1;
            public const byte Spawn = 2;
            public const byte Despawn = 3;
        }
        public static class Udp
        {
            public const byte Snapshot = 0;
        }
    }

    public static class ClientToServer
    {
        public static class Tcp
        {
            public const byte Chat = 0;
            public const byte InitNetwork = 1;
        }
        public static class Udp
        {
            public const byte Input = 0;
        }
    }
}
