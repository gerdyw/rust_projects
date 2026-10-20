using Kanban.Web.Domain.Swimlanes;
using Kanban.Web.Entities;

namespace Kanban.Web.Domain.TaskItems;

public record CreateTaskItemRequest
{
    public required Guid SwimlaneId { get; init; }
    public required string Name { get; init; }
    public string Description { get; init; } = "";
}

public record GetTaskItemResponse
{
    public required Guid Id { get; init; }
    public required string Name { get; init; }
    public required Guid SwimlaneId { get; init; }
    public required int Position { get; init; }
    public required string Description { get; init; }
    public required bool Closed { get; init; }
}

public record UpdateTaskItemRequest
{
    public Guid? SwimlaneId { get; init; }
    public string? Name { get; init; }
    public string? Description { get; init; }
}

public record TaskItemReorderResponse
{
    public required GetSwimlaneDetailResponse DestinationSwimlane { get; init; }
    public List<GetSwimlaneDetailResponse> SourceSwimlanes { get; init; } = [];
}