import os

from dotenv import load_dotenv
from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from langchain_core.messages import HumanMessage
from langchain_openai import ChatOpenAI
from pydantic import BaseModel

load_dotenv()

app = FastAPI()


class ChatRequest(BaseModel):
    message: str


class ChatResponse(BaseModel):
    response: str


@app.post("/chat", response_model=ChatResponse)
async def chat(request: ChatRequest) -> ChatResponse:
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise HTTPException(status_code=500, detail="OPENAI_API_KEY is not set")

    model_name = os.getenv("OPENAI_MODEL", "gpt-5-mini")
    client = ChatOpenAI(model=model_name, api_key=api_key)

    try:
        result = await client.ainvoke([HumanMessage(content=request.message)])
    except Exception as exc:
        raise HTTPException(status_code=500, detail=str(exc)) from exc

    return ChatResponse(response=result.content or "")


@app.post("/chat/stream")
async def chat_stream(request: ChatRequest) -> StreamingResponse:
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise HTTPException(status_code=500, detail="OPENAI_API_KEY is not set")

    model_name = os.getenv("OPENAI_MODEL", "gpt-4o-mini")
    client = ChatOpenAI(model=model_name, api_key=api_key, streaming=True)

    async def generate():
        try:
            async for chunk in client.astream([HumanMessage(content=request.message)]):
                if chunk.content:
                    yield chunk.content
        except Exception as exc:
            yield f"\n[stream_error] {exc}"

    return StreamingResponse(generate(), media_type="text/plain")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
