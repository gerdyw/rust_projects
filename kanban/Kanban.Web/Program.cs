using Kanban.Web;
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
builder.Services.AddRazorComponents();

var app = builder.Build();

app.UseStaticFiles();
app.UseAntiforgery();

app.MapGet("/", () => "Hello World!");

app.MapGroup("/api/v1")
    .MapBoardEndpoints()
    .MapSwimlaneEndpoints()
    .MapTaskItemEndpoints();

app.MapTaskItemViewEndpoints()
    .MapSwimlaneViewEndpoints();

app.MapViewEndpoints();

app.Run();
