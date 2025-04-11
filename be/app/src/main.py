from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager
import asyncio
from fastapi.responses import FileResponse
from fastapi import HTTPException
import pandas as pd

from config import CONFIG
from common.logger import log
from common.camino.eic_summary_people_processor import get_eic2025summary_people_report, get_other_expenses_report


if CONFIG['ENVIRONMENT'] == "develop":
    import debugpy
    debugpy.listen(5678)
    APP_PATH = CONFIG['APP_PATH']
    

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    # task = asyncio.create_task()
    yield
    # Shutdown
    # task.cancel()



app = FastAPI(
    title="Project Manager",
    description="Project Manager API",
    version="0.1.0",
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Allows all origins
    allow_methods=["*"],  # Allows all methods
    allow_headers=["*"],  # Allows all headers
    allow_credentials=True,
)


# app.include_router(events.router)
# app.include_router(slack.router)


@app.get("/health")
def health_check():
    return "ok"


@app.get("/favicon.ico", include_in_schema=False)
def favicon():
    return FileResponse(f"{CONFIG['APP_PATH']}/static/favicon/favicon.ico")




@app.get("/tasks")
def get_tasks():
    return FileResponse(f"{CONFIG['APP_PATH']}/local/tmp/list_of_tasks.json")


@app.get("/eic2025summary-people")
def get_eic2025summary_people():
    dfs = get_eic2025summary_people_report()
    return {
        "costs_report": dfs["costs_report"].to_dict(orient="records"),
        "time_report": dfs["time_report"].to_dict(orient="records")
    }


@app.get("/eic2025summary-other")
def get_eic2025summary_other():
    df = get_other_expenses_report()
    return df.to_dict(orient="records")





@app.get("/project-summary")
def get_planned_summary():
    df = pd.read_json(f"{CONFIG['APP_PATH']}/local/tmp/list_of_tasks.json")
    df['parent_name'] = df['parent'].map(df.set_index('id')['name'])

    work_packages = df.groupby("parent_name").agg({
        "planned_work_pm": "sum",
        "planned_cost_eur": "sum"
    }).reset_index()

    return {
        "planned_work_pm": df["planned_work_pm"].sum(),
        "planned_cost_eur": df["planned_cost_eur"].sum(),
        "work_packages": [
            {
                "name": row["parent_name"],
                "planned_work_pm": row["planned_work_pm"],
                "planned_cost_eur": row["planned_cost_eur"]
            }
            for _, row in work_packages.iterrows()
        ]
    }
