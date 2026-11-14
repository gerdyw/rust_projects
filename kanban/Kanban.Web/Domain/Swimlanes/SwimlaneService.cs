using Kanban.Web.Data;
using Kanban.Web.Entities;
using Microsoft.EntityFrameworkCore;

namespace Kanban.Web.Domain.Swimlanes;

public class SwimlaneService(KanbanDbContext db)
{
    public async Task<List<Swimlane>> GetSwimlanesAsync(Guid boardId, CancellationToken ct) => await db.Swimlanes
            .AsNoTracking()
            .Where(s => s.Board.PublicId == boardId)
            .OrderBy(s => s.Position)
            .ThenBy(s => s.Id)
            .ToListAsync(ct);

    public async Task<Swimlane?> GetSwimlaneAsync(Guid publicId, CancellationToken ct) => await db.Swimlanes
            .AsNoTracking()
            .FirstOrDefaultAsync(s => s.PublicId == publicId, ct);

    public async Task<Swimlane?> GetSwimlaneDetailAsync(Guid publicId, CancellationToken ct) => await db.Swimlanes
            .AsNoTracking()
            .Include(s => s.TaskItems.OrderBy(t => t.Position).ThenBy(t => t.Id))
            .FirstOrDefaultAsync(s => s.PublicId == publicId, ct);

    public async Task<Swimlane?> CreateSwimlaneAsync(Guid boardId, string name, bool closeTask, CancellationToken ct)
    {
        var board = await db.Boards.FirstOrDefaultAsync(b => b.PublicId == boardId, ct);
        if (board is null)
        {
            Console.WriteLine($"Could not find board {boardId}");
            return null;
        }

        var highestPosition = await db.Swimlanes
            .Where(s => s.BoardId == board.Id)
            .OrderBy(s => s.Position)
            .Select(s => s.Position)
            .LastOrDefaultAsync(ct);

        var swimlane = new Swimlane
        {
            Name = name,
            Board = board,
            Position = highestPosition + 1,
            CloseTask = closeTask
        };

        db.Swimlanes.Add(swimlane);
        await db.SaveChangesAsync(ct);
        return swimlane;
    }

    public async Task<Swimlane?> UpdateSwimlaneAsync(Guid publicId, UpdateSwimlaneRequest req, CancellationToken ct)
    {
        var swimlane = await db.Swimlanes.SingleOrDefaultAsync(s => s.PublicId == publicId, ct);
        if (swimlane is null)
        {
            Console.WriteLine($"Could not find swimlane {publicId}");
            return null;
        }

        swimlane.Name = req.Name;
        swimlane.CloseTask = req.CloseTask;
        await db.SaveChangesAsync(ct);
        return swimlane;
    }

    public async Task<DeleteSwimlaneOutcome> DeleteSwimlaneAsync(Guid publicId, CancellationToken ct)
    {
        var lane = await db.Swimlanes
            .Where(s => s.PublicId == publicId)
            .Select(s => new { s.Id, HasTasks = s.TaskItems.Any() })
            .FirstOrDefaultAsync(ct);

        if (lane is null) return DeleteSwimlaneOutcome.NotFound;
        if (lane.HasTasks) return DeleteSwimlaneOutcome.NotEmpty;

        await db.Swimlanes.Where(s => s.Id == lane.Id).ExecuteDeleteAsync(ct);
        return DeleteSwimlaneOutcome.Deleted;
    }

    public async Task<bool> ReorderSwimlanesAsync(Guid boardPublicId, IReadOnlyList<Guid> orderedIds, CancellationToken ct)
    {
        var lanes = await db.Swimlanes
            .Where(s => s.Board.PublicId == boardPublicId)
            .ToDictionaryAsync(s => s.PublicId, ct);

        var isComplete = orderedIds.Count == lanes.Count
            && orderedIds.Distinct().Count() == orderedIds.Count
            && orderedIds.All(lanes.ContainsKey);

        if (!isComplete) return false;

        for (var i = 0; i < orderedIds.Count; i++)
            lanes[orderedIds[i]].Position = i;

        await db.SaveChangesAsync(ct);
        return true;
    }
}

public enum DeleteSwimlaneOutcome
{
    Deleted,
    NotFound,
    NotEmpty
}
