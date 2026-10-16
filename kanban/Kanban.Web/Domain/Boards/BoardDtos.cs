using Kanban.Web.Domain.Swimlanes;

namespace Kanban.Web.Domain.Boards;

public record CreateBoardRequest
{
    public required string Name { get; init; }
}

public record GetBoardResponse
{
    public required Guid Id { get; init; }
    public required string Name { get; init; }
}

public record GetBoardDetailResponse : GetBoardResponse
{
    public required ICollection<GetSwimlaneDetailResponse> Swimlanes { get; init; }
}

public record UpdateBoardRequest
{
    public required string Name { get; init; }
}
