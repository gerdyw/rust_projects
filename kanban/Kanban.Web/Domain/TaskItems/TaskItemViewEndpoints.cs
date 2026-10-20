using Kanban.Web.Common.Dtos;
using Kanban.Web.Domain.Swimlanes.Fragments;
using Kanban.Web.Domain.TaskItems.Fragments;
using Kanban.Web.Entities;
using Microsoft.AspNetCore.Http.HttpResults;
using Microsoft.AspNetCore.Mvc;

namespace Kanban.Web.Domain.TaskItems;

public static class TaskItemViewEndpoints
{
    public readonly static string Root = "/taskitem";
    public readonly static string AddTask = $"{Root}/add";
    public readonly static string MoveTasks = $"{Root}/move";

    public static WebApplication MapTaskItemViewEndpoints(this WebApplication app)
    {
        app.MapGet($"{Root}/{{publicId:guid}}", GetTaskItemAsync);
        app.MapDelete($"{Root}/{{publicId:guid}}", DeleteTaskItemAsync);
        app.MapPost(AddTask, SubmitTaskItemAsync);
        app.MapPost(MoveTasks, MoveTaskItemsAsync);
        return app;
    }

    private static async Task<Results<RazorComponentResult<TaskItemView>, NotFound>> GetTaskItemAsync(
        Guid publicId, TaskItemService service, CancellationToken ct = default
    )
    {
        var taskItem = await service.GetTaskItemAsync(publicId, ct);
        return taskItem is { }
            ? new RazorComponentResult<TaskItemView>(new { TaskItem = taskItem })
            : TypedResults.NotFound();
    }

    private static async Task<Results<RazorComponentResult<AddTaskItemResponse>, BadRequest>> SubmitTaskItemAsync(
       [FromForm] CreateTaskItemRequest req, TaskItemService service, CancellationToken ct = default
    )
    {
        var createdTask = await service.CreateTaskItemAsync(req.SwimlaneId, req.Name, req.Description, ct);

        if (createdTask is null)
        {
            return TypedResults.BadRequest();
        }
        else
        {
            var taskCount = await service.CountTaskItemsForSwimlaneAsync(createdTask.SwimlaneId, ct);

            return new RazorComponentResult<AddTaskItemResponse>(new { TaskItem = createdTask, TaskItemCount = taskCount });
        }
    }

    private static async Task<Results<RazorComponentResult<SwimlaneTaskCount>, NotFound>> DeleteTaskItemAsync(
        Guid publicId, TaskItemService service, CancellationToken ct = default)
    {
        var taskItem = await service.GetTaskItemAsync(publicId, ct);
        if (taskItem is null)
        {
            return TypedResults.NotFound();
        }

        await service.DeleteTaskItemAsync(publicId, ct);
        var taskCount = await service.CountTaskItemsForSwimlaneAsync(taskItem.SwimlaneId, ct);
        return new RazorComponentResult<SwimlaneTaskCount>(new { SwimlaneId = taskItem.Swimlane.PublicId, TaskItemCount = taskCount, SwapOob = true });
    }

    private static async Task<Results<RazorComponentResult<ReorderedTaskItemResponse>, NotFound>> MoveTaskItemsAsync(
        [FromForm] ReorderIdsDto req, TaskItemService service, CancellationToken ct = default
    )
    {
        var outcome = await service.MoveTaskItemsAsync(req.DestParentId, req.Guid, ct);
        if (outcome is not { } result)
        {
            return TypedResults.NotFound();
        }

        (Swimlane destSwimlane, List<Swimlane> sourceSwimlanes) = result;

        return new RazorComponentResult<ReorderedTaskItemResponse>(new { DestSwimlane = destSwimlane, SourceSwimlanes = sourceSwimlanes });
    }
}
