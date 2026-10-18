using Kanban.Web.Domain.Swimlanes;
using Kanban.Web.Domain.TaskItems;
using Kanban.Web.Entities;
using Microsoft.AspNetCore.Http.HttpResults;

namespace Kanban.Web.Domain.Boards;

public static class BoardEndpoints
{
    public static RouteGroupBuilder MapBoardEndpoints(this RouteGroupBuilder api)
    {
        var group = api.MapGroup("/board");
        group.MapGet("/{id:guid}", GetBoard);
        group.MapGet("/detail/{id:guid}", GetBoardDetail);
        group.MapPost("/", CreateBoard);
        group.MapPatch("/{id:guid}", UpdateBoard);
        group.MapDelete("/{id:guid}", DeleteBoard);
        return api;
    }

    private static async Task<Results<Ok<GetBoardResponse>, NotFound>> GetBoard(
        Guid id, BoardService service, CancellationToken ct = default)
    {
        var board = await service.GetBoardAsync(id, ct);

        return board is { }
            ? TypedResults.Ok(ToResponse(board))
            : TypedResults.NotFound();
    }

    private static async Task<Results<Ok<GetBoardDetailResponse>, NotFound>> GetBoardDetail(
        Guid id, BoardService service, CancellationToken ct = default)
    {
        var board = await service.GetBoardDetailAsync(id, ct);

        if (board is null)
        {
            return TypedResults.NotFound();
        }

        var response = new GetBoardDetailResponse
        {
            Id = board.PublicId,
            Name = board.Name,
            Swimlanes = board.Swimlanes.Select(s => new GetSwimlaneDetailResponse
            {
                Id = s.PublicId,
                Name = s.Name,
                CloseTask = s.CloseTask,
                TaskItems = s.TaskItems.Select(
                    t => new GetTaskItemResponse { Id = t.PublicId, Name = t.Name, Description = t.Description, Closed = t.Closed }
                ).ToList()
            }).ToList()
        };

        return TypedResults.Ok(response);
    }

    private static async Task<Created<GetBoardResponse>> CreateBoard(
        CreateBoardRequest req, BoardService service, CancellationToken ct = default)
    {
        var board = await service.CreateBoardAsync(req.Name, ct);
        return TypedResults.Created(ToUri(board), ToResponse(board));
    }

    private static async Task<Results<Accepted<GetBoardResponse>, NotFound>> UpdateBoard(
        Guid id, UpdateBoardRequest req, BoardService service, CancellationToken ct = default)
    {
        var board = await service.UpdateBoardAsync(id, req, ct);

        return board is { }
            ? TypedResults.Accepted(ToUri(board), ToResponse(board))
            : TypedResults.NotFound();

    }

    private static async Task<Results<NoContent, NotFound>> DeleteBoard(Guid id, BoardService service, CancellationToken ct)
    {
        var result = await service.DeleteBoardAsync(id, ct);
        return result ? TypedResults.NoContent() : TypedResults.NotFound();
    }

    private static GetBoardResponse ToResponse(Board board) => new()
    {
        Id = board.PublicId,
        Name = board.Name
    };

    private static string ToUri(Board board) => $"/board/{board.PublicId}";
}
