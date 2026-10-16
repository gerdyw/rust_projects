using Kanban.Web.Data;
using Kanban.Web.Entities;
using Microsoft.EntityFrameworkCore;

namespace Kanban.Web.Domain.Swimlanes;

public class SwimlaneService(KanbanDbContext db)
{
    public async Task<List<Swimlane>> GetSwimlanesAsync(Guid boardId, CancellationToken ct) => await db.Swimlanes
            .AsNoTracking()
            .Where(s => s.Board.PublicId == boardId)
            .ToListAsync(ct);

    public async Task<Swimlane?> GetSwimlaneAsync(Guid publicId, CancellationToken ct) => await db.Swimlanes
            .AsNoTracking()
            .FirstOrDefaultAsync(s => s.PublicId == publicId, ct);

    public async Task<Swimlane?> CreateSwimlaneAsync(Guid boardId, string name, bool closeTask, CancellationToken ct)
    {
        var board = await db.Boards.FirstOrDefaultAsync(b => b.PublicId == boardId, ct);
        if (board is null)
        {
            Console.WriteLine($"Could not find board {boardId}");
            return null;
        }

        var swimlane = new Swimlane
        {
            Board = board,
            Name = name,
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

    public async Task<bool> DeleteSwimlaneAsync(Guid publicId, CancellationToken ct)
    {
        var rows = await db.Swimlanes.Where(s => s.PublicId == publicId).ExecuteDeleteAsync(ct);
        return rows == 1;
    }
}
