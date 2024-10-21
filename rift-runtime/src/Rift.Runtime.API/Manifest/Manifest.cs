using Rift.Runtime.API.Schema;

namespace Rift.Runtime.API.Manifest;

/*
/// 针对项目本身的。
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub enum Manifest {
       Project(ProjectManifest),
       Target(TargetManifest),
   }
   
   impl Manifest {
       pub fn name(&self) -> String {
           match self {
               Manifest::Project(p) => p.name.clone(),
               Manifest::Target(t) => t.name.clone(),
           }
       }
   
       pub fn dependencies(&self) -> Option<String> {
           match self {
               Manifest::Project(p) => p.dependencies.clone(),
               Manifest::Target(t) => t.dependencies.clone(),
           }
       }
       pub fn plugins(&self) -> Option<String> {
           match self {
               Manifest::Project(p) => p.plugins.clone(),
               Manifest::Target(t) => t.plugins.clone(),
           }
       }
       pub fn metadata(&self) -> Option<String> {
           match self {
               Manifest::Project(p) => p.metadata.clone(),
               Manifest::Target(t) => t.metadata.clone(),
           }
       }
}
 */

public class Manifest<T> where T : class
{
    private readonly T _data = null!;
    public Manifest(T data)
    {
        if (data is not TomlProject or TomlTarget)
        {
            return;
        }
        _data = data;
    }

    public string Name
    {
        get
        {
            return _data switch
            {
                TomlProject project => project.Name,
                TomlTarget target => target.Name,
                _ => throw new InvalidOperationException("Only accepts \"TomlProject\" or \"TomlTarget\"")
            };
        }
    }
}