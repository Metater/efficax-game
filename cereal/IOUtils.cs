namespace MetaCereal;

public static class IOUtils
{
    public static string[] ReadSchema(string schema)
    {
        return File.ReadAllLines($@"C:\Users\Connor Myers\Desktop\Projects\efficax-game\schema\{schema}.mc");
    }

    public static void CopyAll(string sourcePath, string targetPath)
    {
        foreach (string dirPath in Directory.GetDirectories(sourcePath, "*", SearchOption.AllDirectories))
        {
            Directory.CreateDirectory(dirPath.Replace(sourcePath, targetPath));
        }

        foreach (string newPath in Directory.GetFiles(sourcePath, "*.*", SearchOption.AllDirectories))
        {
            File.Copy(newPath, newPath.Replace(sourcePath, targetPath), true);
        }
    }


}
