using System.ComponentModel.DataAnnotations.Schema;

namespace Kanban.Web.Entities;

[Table("taskitems")]
public class TaskItem : BaseEntity
{
    public int SwimlaneId { get; set; }
    public required Swimlane Swimlane { get; set; }
    public required string Name { get; set; }
    public string Description { get; set; } = "";
    public bool Closed { get; set; } = false;
}
