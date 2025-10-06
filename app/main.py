# sentiric-stt-gateway-service/app/main.py
from fastapi import FastAPI, Depends, HTTPException, status
from contextlib import asynccontextmanager
from app.core.logging import setup_logging
from app.core.config import settings
import structlog

logger = structlog.get_logger(__name__)

@asynccontextmanager
async def lifespan(app: FastAPI):
    setup_logging()
    logger.info("STT Gateway Service başlatılıyor", version=settings.SERVICE_VERSION, env=settings.ENV)
    
    # TODO: gRPC istemcileri (Whisper, Google STT, vb.) burada başlatılacak.
    
    yield
    
    logger.info("STT Gateway Service kapatılıyor")

app = FastAPI(
    title="Sentiric STT Gateway Service",
    description="Akıllı STT Yönlendiricisi",
    version=settings.SERVICE_VERSION,
    lifespan=lifespan
)

# STT Routing endpoint'leri (Transcribe ve Streaming) burada tanımlanacak veya import edilecek.

@app.get("/health", status_code=status.HTTP_200_OK)
async def health_check():
    # Placeholder: Sadece sunucunun ayakta olduğunu kontrol eder.
    return {"status": "ok", "service": "stt-gateway"}

# --- Placeholder WebSocket ve Transcribe endpoint'leri ---
# Not: Gerçek implementasyon daha karmaşık olacaktır.

# @app.post(settings.API_V1_STR + "/transcribe")
# async def transcribe_file(file: UploadFile):
#     # ... Yönlendirme Mantığı ...
#     return {"transcription": "Simulated result."}

# @app.websocket("/ws/stream")
# async def websocket_endpoint(websocket: WebSocket):
#     # ... Streaming Mantığı ...
#     pass