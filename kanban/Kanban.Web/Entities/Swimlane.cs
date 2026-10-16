using System.ComponentModel.DataAnnotations.Schema;

namespace Kanban.Web.Entities;

[Table("swimlanes")]
public class Swimlane : BaseEntity
{
    public int BoardId { get; set; }
    public required Board Board { get; set; }
    public required string Name { get; set; }
    public required bool CloseTask { get; set; }
    public ICollection<TaskItem> TaskItems { get; set; } = [];
}
