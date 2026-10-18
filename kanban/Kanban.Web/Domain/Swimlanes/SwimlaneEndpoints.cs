using Kanban.Web.Entities;
using Microsoft.AspNetCore.Http.HttpResults;

namespace Kanban.Web.Domain.Swimlanes;

public static class SwimlaneEndpoints
{
    public static RouteGroupBuilder MapSwimlaneEndpoints(this RouteGroupBuilder api)
    {
        var group = api.MapGroup("/swimlane");
        group.MapGet("/", GetSwimlanes);
        group.MapGet("/{id:guid}", GetSwimlane);
        group.MapPost("/", CreateSwimlane);
        group.MapPatch("/{id:guid}", UpdateSwimlane);
        group.MapDelete("/{id:guid}", DeleteSwimlane);
        return api;
    }

    private static async Task<Ok<List<GetSwimlaneResponse>>> GetSwimlanes(
        Guid boardId, SwimlaneService service, CancellationToken ct = default)
    {
        var swimlanes = await service.GetSwimlanesAsync(boardId, ct);
        return TypedResults.Ok(swimlanes.Select(ToResponse).ToList());
    }

    private static async Task<Results<Ok<GetSwimlaneResponse>, NotFound>> GetSwimlane(
        Guid id, SwimlaneService service, CancellationToken ct = default)
    {
        var swimlane = await service.GetSwimlaneAsync(id, ct);

        return swimlane is { }
            ? TypedResults.Ok(ToResponse(swimlane))
            : TypedResults.NotFound();
    }

    private static async Task<Results<Created<GetSwimlaneResponse>, ValidationProblem>> CreateSwimlane(
        CreateSwimlaneRequest req, SwimlaneService service, CancellationToken ct = default)
    {
        var swimlane = await service.CreateSwimlaneAsync(req.BoardId, req.Name, req.CloseTask, ct);

        return swimlane is { }
            ? TypedResults.Created(ToUri(swimlane), ToResponse(swimlane))
            : TypedResults.ValidationProblem(new Dictionary<string, string[]>
            {
                [nameof(req.BoardId)] = [$"Board {req.BoardId} does not exist"]
            });
    }

    private static async Task<Results<Accepted<GetSwimlaneResponse>, NotFound>> UpdateSwimlane(
        Guid id, UpdateSwimlaneRequest req, SwimlaneService service, CancellationToken ct = default)
    {
        var swimlane = await service.UpdateSwimlaneAsync(id, req, ct);

        return swimlane is { }
            ? TypedResults.Accepted(ToUri(swimlane), ToResponse(swimlane))
            : TypedResults.NotFound();
    }

    private static async Task<Results<NoContent, NotFound>> DeleteSwimlane(
        Guid id, SwimlaneService service, CancellationToken ct)
    {
        var result = await service.DeleteSwimlaneAsync(id, ct);
        return result ? TypedResults.NoContent() : TypedResults.NotFound();
    }

    private static GetSwimlaneResponse ToResponse(Swimlane swimlane) => new()
    {
        Id = swimlane.PublicId,
        Name = swimlane.Name,
        CloseTask = swimlane.CloseTask
    };

    private static string ToUri(Swimlane swimlane) => $"/swimlane/{swimlane.PublicId}";
}
