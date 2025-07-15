from llama_cpp import Llama

from dataclasses import dataclass
from typing import List, Optional
import time

@dataclass
class Message:
    role: str
    content: str

@dataclass
class Choice:
    index: int
    message: Message
    logprobs: Optional[dict]
    finish_reason: Optional[str]

@dataclass
class Usage:
    prompt_tokens: int
    completion_tokens: int
    total_tokens: int

@dataclass
class ChatCompletion:
    id: str
    object: str
    created: int
    model: str
    choices: List[Choice]
    usage: Usage

    @classmethod
    def from_llm_response(cls, resp) -> "ChatCompletion":
        created = getattr(resp, "created", int(time.time()))

        # build usage
        usage = Usage(
            prompt_tokens=resp.usage.prompt_tokens,
            completion_tokens=resp.usage.completion_tokens,
            total_tokens=resp.usage.total_tokens
        )

        # build choices
        choices = []
        for ch in resp.choices:
            msg = Message(
                role=ch.message.role,
                content=ch.message.content
            )
            choice = Choice(
                index=ch.index,
                message=msg,
                logprobs=getattr(ch, "logprobs", None),
                finish_reason=ch.finish_reason
            )
            choices.append(choice)

        return cls(
            id=resp.id,
            object=getattr(resp, "object", "chat.completion"),
            created=created,
            model=resp.model,
            choices=choices,
            usage=usage
        )
def run_local_model():
    llm = Llama.from_pretrained(
        repo_id="Qwen/Qwen2-0.5B-Instruct-GGUF",
        filename="*q8_0.gguf",
        verbose=False
    )
    reponse = llm.create_chat_completion(
        messages = [
            {"role": "system", "content": "You are an assistant who perfectly describes images."},
            {
                "role": "user",
                "content": "salut ma beaute"
            }
        ]
    )
    print(reponse)
