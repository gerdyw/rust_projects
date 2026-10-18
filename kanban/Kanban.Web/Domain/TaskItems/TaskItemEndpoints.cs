using Kanban.Web.Entities;
using Microsoft.AspNetCore.Http.HttpResults;

namespace Kanban.Web.Domain.TaskItems;

public static class TaskItemEndpoints
{
    public static RouteGroupBuilder MapTaskItemEndpoints(this RouteGroupBuilder api)
    {
        var group = api.MapGroup("/task");
        group.MapGet("/", GetTaskItems);
        group.MapGet("/{publicId:guid}", GetTaskItem);
        group.MapPost("/", CreateTaskItem);
        group.MapPatch("/{publicId:guid}", UpdateTaskItem);
        group.MapDelete("/{publicId:guid}", DeleteTaskItem);
        return api;
    }

    private static async Task<Ok<List<GetTaskItemResponse>>> GetTaskItems(
        Guid swimlaneId, TaskItemService service, CancellationToken ct = default)
    {
        var taskItems = await service.GetTaskItemsAsync(swimlaneId, ct);
        return TypedResults.Ok(taskItems.Select(ToResponse).ToList());
    }

    private static async Task<Results<Ok<GetTaskItemResponse>, NotFound>> GetTaskItem(
        Guid publicId, TaskItemService service, CancellationToken ct = default)
    {
        var taskItem = await service.GetTaskItemAsync(publicId, ct);

        return taskItem is { }
            ? TypedResults.Ok(ToResponse(taskItem))
            : TypedResults.NotFound();
    }

    private static async Task<Results<Created<GetTaskItemResponse>, ValidationProblem>> CreateTaskItem(
        CreateTaskItemRequest req, TaskItemService service, CancellationToken ct = default)
    {
        var taskItem = await service.CreateTaskItemAsync(req.SwimlaneId, req.Name, req.Description, ct);

        return taskItem is { }
            ? TypedResults.Created(ToUri(taskItem), ToResponse(taskItem))
            : TypedResults.ValidationProblem(new Dictionary<string, string[]>
            {
                [nameof(req.SwimlaneId)] = [$"Swimlane {req.SwimlaneId} does not exist"]
            });
    }

    private static async Task<Results<Accepted<GetTaskItemResponse>, NotFound>> UpdateTaskItem(
        Guid publicId, UpdateTaskItemRequest req, TaskItemService service, CancellationToken ct = default)
    {
        var taskItem = await service.UpdateTaskItemAsync(publicId, req, ct);

        return taskItem is { }
            ? TypedResults.Accepted(ToUri(taskItem), ToResponse(taskItem))
            : TypedResults.NotFound();
    }

    private static async Task<Results<NoContent, NotFound>> DeleteTaskItem(
        Guid publicId, TaskItemService service, CancellationToken ct)
    {
        var result = await service.DeleteTaskItemAsync(publicId, ct);
        return result ? TypedResults.NoContent() : TypedResults.NotFound();
    }

    private static GetTaskItemResponse ToResponse(TaskItem taskItem) => new()
    {
        Id = taskItem.PublicId,
        Name = taskItem.Name,
        Description = taskItem.Description,
        Closed = taskItem.Closed
    };

    private static string ToUri(TaskItem taskItem) => $"/task/{taskItem.PublicId}";
}
