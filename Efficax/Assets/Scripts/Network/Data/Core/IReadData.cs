using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

public interface IReadData<T>
{
    public T Read(NetDataReader reader, byte tickId);
}