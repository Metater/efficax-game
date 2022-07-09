using System.Text;

Console.WriteLine("Hello, World!\n");

StringBuilder RsTcpCSNetworkDataEnum = new();
RsTcpCSNetworkDataEnum.AppendLine("#[derive(Debug)]");
RsTcpCSNetworkDataEnum.AppendLine("pub enum NetworkData {");

Dictionary<string, StringBuilder> networkDataEnums = new();
networkDataEnums["rs-tcp-client-to-server"] = RsTcpCSNetworkDataEnum;

#region E
var main = IOUtils.ReadSchema("main");

int cursor = 0;
string? parent = null;
while (true)
{
    if (cursor >= main!.Length)
    {
        break;
    }

    string line = main![cursor++];

    if (string.IsNullOrWhiteSpace(line))
    {
        continue;
    }

    switch (line)
    {
        case "tcp-client-to-server" or "tcp-server-to-client" or "udp-client-to-server" or "udp-server-to-client":
            parent = line;
            //Console.WriteLine(parent);
            break;
        default:
            if (parent is null)
            {
                Console.WriteLine("error: no valid first container parent");
            }
            else
            {
                string schemaName = line.Trim();
                char firstChar = schemaName[0];
                string[] schema;
                if (firstChar == '$')
                {
                    schemaName = schemaName[1..];
                    schema = IOUtils.ReadSchema($"data/shared/{schemaName}");
                }
                else
                {
                    schema = IOUtils.ReadSchema($"data/{parent}/{schemaName}");
                }

                //Console.WriteLine($"\t{schemaName}");
                HandleSchema(parent, schemaName, schema);
            }
            break;
    }
}
#endregion E

void HandleSchema(string parent, string schemaName, string[] schema)
{
    foreach (var item in schema)
    {
        //Console.WriteLine($"\t\t{item}");
    }

    string pascalSchemaName = ToPascal(schemaName);
    networkDataEnums["rs-" + parent].AppendLine($"\t{pascalSchemaName}({pascalSchemaName}Data),");

    switch (parent)
    {
        case "tcp-client-to-server":
            break;
        case "tcp-server-to-client":
            break;
        case "udp-client-to-server":
            break;
        case "udp-server-to-client":
            break;
    }
}

static string ToPascal(string input)
{
    string output = "";
    string[] segments = input.Split('-');
    foreach (string segment in segments)
    {
        output += char.ToUpper(segment[0]);
        output += segment[1..];
    }
    return output;
}

static string ToCamel(string input)
{
    string pascal = ToPascal(input);
    string end = pascal[1..];
    char start = char.ToLower(pascal[0]);
    return start + end;
}

static string ToSnake(string input)
{
    return input.Replace('-', '_');
}

RsTcpCSNetworkDataEnum.AppendLine("}");

Console.WriteLine(networkDataEnums["rs-tcp-client-to-server"].ToString());