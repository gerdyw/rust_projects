using Kanban.Web.Common;
using Kanban.Web.Domain.Boards;
using Microsoft.AspNetCore.Http.HttpResults;

namespace Kanban.Web;

public static class RootViewEndpoints
{
    public static WebApplication MapViewEndpoints(this WebApplication app)
    {
        app.MapGet("/{publicId:guid}", GetBoardPage);
        return app;
    }
    static async Task<Results<RazorComponentResult<Homepage>, NotFound>> GetBoardPage(
    Guid publicId, BoardService service, CancellationToken ct)
    {
        var board = await service.GetBoardDetailAsync(publicId, ct);
        return board is { }
            ? new RazorComponentResult<Homepage>(new { Board = board })
            : TypedResults.NotFound();
    }
}