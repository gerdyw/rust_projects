using Kanban.Web.Domain.Swimlanes.Fragments;
using Kanban.Web.Entities;
using Microsoft.AspNetCore.Http.HttpResults;
using Microsoft.AspNetCore.Mvc;

namespace Kanban.Web.Domain.Swimlanes;

public static class SwimlaneViewEndpoints
{
    public static WebApplication MapSwimlaneViewEndpoints(this WebApplication app)
    {
        var group = app.MapGroup("/swimlane");
        group.MapGet("/{publicId:guid}", GetSwimlaneAsync);
        group.MapGet("/add", GetNewSwimlaneForm);
        group.MapPost("/add", AddSwimlaneAsync);
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

    private static async Task<RazorComponentResult<NewSwimlaneForm>> GetNewSwimlaneForm(
        Guid boardId, CancellationToken ct = default
    )
    {
        return new(new { BoardId = boardId, SubmitEndpoint = "/swimlane/add" });
    }

    private static async Task<Results<RazorComponentResult<CreateSwimlaneResponse>, BadRequest>> AddSwimlaneAsync(
        [FromForm] CreateSwimlaneRequest req, SwimlaneService service, CancellationToken ct = default
    )
    {
        var swimlane = await service.CreateSwimlaneAsync(req.BoardId, req.Name, req.CloseTask, ct);
        return swimlane is { }
            ? new RazorComponentResult<CreateSwimlaneResponse>(new { Swimlane = swimlane, req.BoardId })
            : TypedResults.BadRequest();
    }
}