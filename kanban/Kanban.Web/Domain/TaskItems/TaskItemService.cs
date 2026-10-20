using Kanban.Web.Data;
using Kanban.Web.Entities;
using Microsoft.EntityFrameworkCore;

namespace Kanban.Web.Domain.TaskItems;

public class TaskItemService(KanbanDbContext db)
{
    public async Task<List<TaskItem>> GetTaskItemsAsync(Guid swimlaneId, CancellationToken ct) => await db.TaskItems
            .AsNoTracking()
            .Include(t => t.Swimlane)
            .Where(t => t.Swimlane.PublicId == swimlaneId)
            .OrderBy(t => t.Position)
            .ThenBy(t => t.Id)
            .ToListAsync(ct);

    public async Task<TaskItem?> GetTaskItemAsync(Guid publicId, CancellationToken ct) =>
        await db.TaskItems.AsNoTracking()
            .Include(t => t.Swimlane)
            .FirstOrDefaultAsync(t => t.PublicId == publicId, ct);

    public async Task<TaskItem?> CreateTaskItemAsync(Guid swimlaneId, string name, string description, CancellationToken ct)
    {
        var swimlane = await db.Swimlanes.FirstOrDefaultAsync(s => s.PublicId == swimlaneId, ct);
        if (swimlane is null)
        {
            Console.WriteLine($"Could not find swimlane {swimlaneId}");
            return null;
        }

        var highestPosition = await db.TaskItems
            .Where(t => t.SwimlaneId == swimlane.Id)
            .OrderBy(t => t.Position)
            .Select(t => t.Position)
            .LastOrDefaultAsync(ct);

        var taskItem = new TaskItem
        {
            Name = name,
            Swimlane = swimlane,
            Position = highestPosition + 1,
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

    public async Task<(Swimlane, List<Swimlane>)?> MoveTaskItemsAsync(Guid destSwimlaneId, IEnumerable<Guid> taskItemIds, CancellationToken ct)
    {
        var destSwimlane = await db.Swimlanes.FirstOrDefaultAsync(s => s.PublicId == destSwimlaneId, ct);

        if (destSwimlane is null) return null;

        var taskItems = await db.TaskItems.Where(t => taskItemIds
            .Contains(t.PublicId))
            .Include(t => t.Swimlane)
            .ThenInclude(s => s.TaskItems)
            .ToDictionaryAsync(t => t.PublicId, ct);

        var swimlaneIds = taskItems.Values.Select(t => t.Swimlane.PublicId).ToHashSet();
        swimlaneIds.Add(destSwimlane.PublicId);

        foreach (var (id, idx) in taskItemIds.Select((id, idx) => (id, idx)))
        {
            if (taskItems.TryGetValue(id, out var taskItem))
            {
                taskItem.Position = idx;
                taskItem.Swimlane = destSwimlane;
            }
            else
            {
                return null;
            }
        }

        await db.SaveChangesAsync(ct);

        var swimlanes = await db.Swimlanes
            .AsNoTracking()
            .Where(s => swimlaneIds.Contains(s.PublicId))
            .Include(s => s.TaskItems.OrderBy(t => t.Position).ThenBy(t => t.Id))
            .ToListAsync(ct);
        destSwimlane = swimlanes.First(s => s.PublicId == destSwimlaneId);
        var sourceSwimlanes = swimlanes.Where(s => s.PublicId != destSwimlaneId).ToList();

        return (destSwimlane, sourceSwimlanes);
    }

    public async Task<bool> DeleteTaskItemAsync(Guid publicId, CancellationToken ct)
    {
        var rows = await db.TaskItems.Where(t => t.PublicId == publicId).ExecuteDeleteAsync(ct);
        return rows == 1;
    }

    public async Task<int> CountTaskItemsForSwimlaneAsync(int swimlaneId, CancellationToken ct) =>
        await db.TaskItems.CountAsync(i => i.Swimlane.Id == swimlaneId, ct);
}
