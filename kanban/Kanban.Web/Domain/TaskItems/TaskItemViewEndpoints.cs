using Kanban.Web.Domain.Boards;
using Kanban.Web.Domain.Swimlanes;
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

    public static WebApplication MapTaskItemViewEndpoints(this WebApplication app)
    {
        app.MapGet($"{Root}/{{publicId:guid}}", GetTaskItemAsync);
        app.MapDelete($"{Root}/{{publicId:guid}}", DeleteTaskItemAsync);
        app.MapGet(AddTask, GetAddTaskFormAsync);
        app.MapPost(AddTask, SubmitTaskItemAsync);
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

    private static async Task<RazorComponentResult<NewTaskItem>> GetAddTaskFormAsync(
        Guid swimlaneId, CancellationToken ct = default
    ) => new(new { SubmitEndpoint = AddTask, SwimlaneId = swimlaneId });

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

    private static async Task<Results<RazorComponentResult<SwimlaneTaskCount>, NotFound>> DeleteTaskItemAsync(Guid publicId, TaskItemService service, CancellationToken ct = default)
    {
        var taskItem = await service.GetTaskItemDetailsAsync(publicId, ct);
        if (taskItem is null)
        {
            return TypedResults.NotFound();
        }

        await service.DeleteTaskItemAsync(publicId, ct);
        var taskCount = await service.CountTaskItemsForSwimlaneAsync(taskItem.SwimlaneId, ct);
        return new RazorComponentResult<SwimlaneTaskCount>(new { SwimlaneId = taskItem.Swimlane.PublicId, TaskItemCount = taskCount, SwapOob = true });
    }
}
