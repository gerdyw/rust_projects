namespace Kanban.Web.Common.Dtos;

public record ReorderLanesRequest(Guid BoardId, List<Guid> LaneIds);
