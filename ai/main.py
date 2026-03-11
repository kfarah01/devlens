from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI(title="DevLens AI Service")

@app.get("/health")
def health():
    return {"status": "ok", "service": "devlens-ai"}