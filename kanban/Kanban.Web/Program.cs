using Kanban.Web.Data;
using Kanban.Web.Domain.Boards;
using Kanban.Web.Domain.Swimlanes;
using Kanban.Web.Domain.TaskItems;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddDbContext<KanbanDbContext>(options =>
    options.UseNpgsql(builder.Configuration.GetConnectionString("DefaultConnection")));
builder.Services.AddScoped<BoardService>();
builder.Services.AddScoped<SwimlaneService>();
builder.Services.AddScoped<TaskItemService>();
builder.Services.AddValidation();

var app = builder.Build();

app.MapGet("/", () => "Hello World!");
var api = app.MapGroup("/api/v1");
api.MapBoardEndpoints();
api.MapSwimlaneEndpoints();
api.MapTaskItemEndpoints();

app.Run();
