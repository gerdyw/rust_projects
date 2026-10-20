using Kanban.Web.Data;
using Kanban.Web.Entities;
using Microsoft.EntityFrameworkCore;

namespace Kanban.Web.Domain.Boards;

public class BoardService(KanbanDbContext db)
{
    public async Task<Board?> GetBoardAsync(Guid publicId, CancellationToken ct)
    {
        Console.WriteLine($"publicId: {publicId}");
        return await db.Boards
            .AsNoTracking()
            .FirstOrDefaultAsync(b => b.PublicId == publicId, ct);
    }

    public async Task<Board?> GetBoardDetailAsync(Guid publicId, CancellationToken ct) =>
        await db.Boards
            .AsNoTracking()
            .Include(b => b.Swimlanes.OrderBy(s => s.Position).ThenBy(s => s.Id))
            .ThenInclude(s => s.TaskItems.OrderBy(t => t.Position).ThenBy(t => t.Id))
            .FirstOrDefaultAsync(b => b.PublicId == publicId, ct);

    public async Task<Board> CreateBoardAsync(string name, CancellationToken ct)
    {
        var board = new Board
        {
            Name = name
        };

        db.Boards.Add(board);
        await db.SaveChangesAsync(ct);
        return board;
    }

    public async Task<Board?> UpdateBoardAsync(Guid publicId, UpdateBoardRequest req, CancellationToken ct)
    {
        var board = await db.Boards.SingleOrDefaultAsync(b => b.PublicId == publicId, ct);
        if (board is null)
        {
            Console.WriteLine($"Could not find board {publicId}");
            return null;
        }

        board.Name = req.Name;
        await db.SaveChangesAsync(ct);
        return board;
    }

    public async Task<bool> DeleteBoardAsync(Guid publicId, CancellationToken ct)
    {
        var rows = await db.Boards.Where(b => b.PublicId == publicId).ExecuteDeleteAsync(ct);
        return rows == 1;
    }
}
