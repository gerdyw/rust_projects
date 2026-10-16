using Microsoft.EntityFrameworkCore;
using Kanban.Web.Entities;

namespace Kanban.Web.Data;

public class KanbanDbContext : DbContext
{
    public KanbanDbContext(DbContextOptions options) : base(options)
    {
    }

    protected KanbanDbContext()
    {
    }

    public DbSet<Board> Boards { get; set; }
    public DbSet<Swimlane> Swimlanes { get; set; }
    public DbSet<TaskItem> TaskItems { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        foreach (var entityType in modelBuilder.Model.GetEntityTypes()
            .Where(t => typeof(BaseEntity).IsAssignableFrom(t.ClrType)))
        {
            modelBuilder.Entity(entityType.ClrType)
                .Property(nameof(BaseEntity.Created))
                .HasDefaultValueSql("now()");
        }
    }
}
