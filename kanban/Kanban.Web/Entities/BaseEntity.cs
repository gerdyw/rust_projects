using System.ComponentModel.DataAnnotations.Schema;
using Microsoft.EntityFrameworkCore;

namespace Kanban.Web.Entities;

[Index(nameof(PublicId), IsUnique = true)]
public abstract class BaseEntity
{
    public int Id { get; set; }
    public Guid PublicId { get; set; } = Guid.CreateVersion7();
    public DateTime Created { get; set; } = DateTime.UtcNow;
}
