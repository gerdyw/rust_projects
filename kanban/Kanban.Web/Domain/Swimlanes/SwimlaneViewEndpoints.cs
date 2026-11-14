using Kanban.Web.Common.Dtos;
using Kanban.Web.Domain.Swimlanes.Fragments;
using Kanban.Web.Entities;
using Microsoft.AspNetCore.Http.HttpResults;
using Microsoft.AspNetCore.Mvc;

namespace Kanban.Web.Domain.Swimlanes;

public static class SwimlaneViewEndpoints
{
    public const string Root = "/swimlane";
    public const string MoveLanes = "/swimlane/move";

    public static WebApplication MapSwimlaneViewEndpoints(this WebApplication app)
    {
        var group = app.MapGroup(Root);
        group.MapGet("/{publicId:guid}", GetSwimlaneAsync);
        group.MapPost("/add", AddSwimlaneAsync);
        group.MapPost("/move", MoveSwimlanesAsync);
        group.MapDelete("/{publicId:guid}", DeleteSwimlaneAsync);
        return app;
    }

    private static async Task<Results<RazorComponentResult<SwimlaneView>, NotFound>> GetSwimlaneAsync(
        Guid publicId, SwimlaneService service, CancellationToken ct = default
    )
    {
        var swimlane = await service.GetSwimlaneDetailAsync(publicId, ct);
        return swimlane is { }
            ? new RazorComponentResult<SwimlaneView>(new { Swimlane = swimlane })
            : TypedResults.NotFound();
    }

    private static async Task<Results<RazorComponentResult<SwimlaneView>, BadRequest>> AddSwimlaneAsync(
        [FromForm] CreateSwimlaneRequest req, SwimlaneService service, CancellationToken ct = default
    )
    {
        var swimlane = await service.CreateSwimlaneAsync(req.BoardId, req.Name, req.CloseTask, ct);
        return swimlane is { }
            ? new RazorComponentResult<SwimlaneView>(new { Swimlane = swimlane })
            : TypedResults.BadRequest();
    }

    private static async Task<Results<NoContent, BadRequest>> MoveSwimlanesAsync(
        [FromBody] ReorderLanesRequest req, SwimlaneService service, CancellationToken ct = default
    )
    {
        var reordered = await service.ReorderSwimlanesAsync(req.BoardId, req.LaneIds, ct);
        return reordered ? TypedResults.NoContent() : TypedResults.BadRequest();
    }

    private static async Task<Results<Ok, NotFound, Conflict>> DeleteSwimlaneAsync(
        Guid publicId, SwimlaneService service, CancellationToken ct = default
    )
    {
        var outcome = await service.DeleteSwimlaneAsync(publicId, ct);

        // 200, not 204: htmx's default responseHandling has swap:false for 204,
        // so hx-swap="delete" would never run against a NoContent response.
        if (outcome == DeleteSwimlaneOutcome.Deleted) return TypedResults.Ok();
        if (outcome == DeleteSwimlaneOutcome.NotEmpty) return TypedResults.Conflict();
        return TypedResults.NotFound();
    }
}
