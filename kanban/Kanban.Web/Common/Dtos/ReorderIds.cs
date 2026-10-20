namespace Kanban.Web.Common.Dtos;

public record ReorderIdsDto
{
    public required Guid DestParentId { get; init; }
    public required List<Guid> Guid { get; init; }
}
