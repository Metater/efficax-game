public static class Network
{
    public static class ServerToClient
    {
        public enum Tcp : byte
        {
            Chat,
            Join,
            Spawn,
            Despawn,
        }
        public enum Udp : byte
        {
            Snapshot,
        }
    }

    public static class ClientToServer
    {
        public enum Tcp : byte
        {
            Chat,
            InitNetwork,
        }
        public enum Udp : byte
        {
            Input,
        }
    }

    public static byte AsByte(this ServerToClient.Tcp tcp) => (byte)tcp;
    public static byte AsByte(this ServerToClient.Udp udp) => (byte)udp;
    public static byte AsByte(this ClientToServer.Tcp tcp) => (byte)tcp;
    public static byte AsByte(this ClientToServer.Udp udp) => (byte)udp;
}
