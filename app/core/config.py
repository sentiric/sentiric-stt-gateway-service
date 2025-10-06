# sentiric-stt-gateway-service/app/core/config.py
from pydantic_settings import BaseSettings, SettingsConfigDict
from typing import Optional

class Settings(BaseSettings):
    PROJECT_NAME: str = "Sentiric STT Gateway Service"
    API_V1_STR: str = "/api/v1"
    
    ENV: str = "production"
    LOG_LEVEL: str = "INFO"
    SERVICE_VERSION: str = "0.1.0"
    
    # STT Engine URL'leri 
    STT_WHISPER_SERVICE_URL: str
    STT_GOOGLE_SERVICE_URL: Optional[str] = None
    STT_STREAMING_SERVICE_URL: Optional[str] = None # Hızlı, akış bazlı STT
    
    # Redis (Caching için)
    REDIS_URL: str

    model_config = SettingsConfigDict(
        env_file=".env", 
        env_file_encoding='utf-8',
        extra='ignore'
    )

settings = Settings()