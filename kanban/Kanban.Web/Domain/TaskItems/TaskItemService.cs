using Kanban.Web.Data;
using Kanban.Web.Entities;
using Microsoft.EntityFrameworkCore;

namespace Kanban.Web.Domain.TaskItems;

public class TaskItemService(KanbanDbContext db)
{
    public async Task<List<TaskItem>> GetTaskItemsAsync(Guid swimlaneId, CancellationToken ct) => await db.TaskItems
            .AsNoTracking()
            .Where(t => t.Swimlane.PublicId == swimlaneId)
            .ToListAsync(ct);

    public async Task<TaskItem?> GetTaskItemAsync(Guid publicId, CancellationToken ct) => await db.TaskItems
            .AsNoTracking()
            .FirstOrDefaultAsync(t => t.PublicId == publicId, ct);

    public async Task<TaskItem?> CreateTaskItemAsync(Guid swimlaneId, string name, string description, CancellationToken ct)
    {
        var swimlane = await db.Swimlanes.FirstOrDefaultAsync(s => s.PublicId == swimlaneId, ct);
        if (swimlane is null)
        {
            Console.WriteLine($"Could not find swimlane {swimlaneId}");
            return null;
        }

        var taskItem = new TaskItem
        {
            Swimlane = swimlane,
            Name = name,
            Description = description,
            Closed = swimlane.CloseTask
        };

        db.TaskItems.Add(taskItem);
        await db.SaveChangesAsync(ct);
        return taskItem;
    }

    public async Task<TaskItem?> UpdateTaskItemAsync(Guid publicId, UpdateTaskItemRequest req, CancellationToken ct)
    {
        var taskItem = await db.TaskItems.SingleOrDefaultAsync(t => t.PublicId == publicId, ct);
        if (taskItem is null)
        {
            Console.WriteLine($"Could not find task item {publicId}");
            return null;
        }

        if (req.SwimlaneId is not null)
        {
            var newSwimlane = await db.Swimlanes.AsNoTracking().SingleOrDefaultAsync(s => s.PublicId == req.SwimlaneId, ct);
            if (newSwimlane is null) return null;
            taskItem.SwimlaneId = newSwimlane.Id;
            taskItem.Closed = newSwimlane.CloseTask;
        }

        taskItem.Name = req.Name ?? taskItem.Name;
        taskItem.Description = req.Description ?? taskItem.Description;
        await db.SaveChangesAsync(ct);
        return taskItem;
    }

    public async Task<bool> DeleteTaskItemAsync(Guid publicId, CancellationToken ct)
    {
        var rows = await db.TaskItems.Where(t => t.PublicId == publicId).ExecuteDeleteAsync(ct);
        return rows == 1;
    }
}
