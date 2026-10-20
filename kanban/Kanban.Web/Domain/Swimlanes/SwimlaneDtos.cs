using Kanban.Web.Domain.TaskItems;

namespace Kanban.Web.Domain.Swimlanes;

public record CreateSwimlaneRequest
{
    public required Guid BoardId { get; init; }
    public required string Name { get; init; }
    public bool CloseTask { get; init; }
}

public record GetSwimlaneResponse
{
    public required Guid Id { get; init; }
    public required string Name { get; init; }
    // public required Guid BoardId { get; init; }
    // public required int Position { get; init; }
    public required bool CloseTask { get; init; }
}

public record GetSwimlaneDetailResponse : GetSwimlaneResponse
{
    public required ICollection<GetTaskItemResponse> TaskItems { get; init; }
}

public record UpdateSwimlaneRequest
{
    public required string Name { get; init; }
    public required bool CloseTask { get; init; }
}
