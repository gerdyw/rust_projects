using System.ComponentModel.DataAnnotations.Schema;

namespace Kanban.Web.Entities;

[Table("boards")]
public class Board : BaseEntity
{
    public required string Name { get; set; }
    public ICollection<Swimlane> Swimlanes { get; set; } = [];
}
