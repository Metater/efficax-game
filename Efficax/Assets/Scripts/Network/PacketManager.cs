using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PacketManager : MonoBehaviour
{
    public GameManager gameManager;

    private void Start()
    {
        
    }

    private void Update()
    {
        
    }

    public void Handle(NetDataReader reader)
    {
        byte packetType = reader.GetByte();
        switch (packetType)
        {
            case 2:
                HandleEntityUpdate(reader);
                break;
            default:
                print($"Unknown packet type: {packetType}");
                break;
        }
    }

    private void HandleEntityUpdate(NetDataReader reader)
    {
        EntityUpdateData data = new EntityUpdateData().Read(reader);
        gameManager.entityManager.UpdateEntity(data);
    }
}
