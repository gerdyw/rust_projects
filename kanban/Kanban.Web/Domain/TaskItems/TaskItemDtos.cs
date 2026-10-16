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
    public required string Description { get; init; }
    public required bool Closed { get; init; }
}

public record UpdateTaskItemRequest
{
    public Guid? SwimlaneId { get; init; }
    public string? Name { get; init; }
    public string? Description { get; init; }
}
