from fastapi import FastAPI
from router.ad import Router as AdRouter


app = FastAPI()

app.include_router(AdRouter)
