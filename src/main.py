from llama_cpp import Llama

if __name__ == "__main__":
    llm = Llama.from_pretrained(
        repo_id="flyfishstudio/qwen2-0_5b-instruct-Q4_0_4_8.gguf",
        filename="qwen2-0_5b-instruct-Q4_0_4_8.gguf",
    )
    llm.create_chat_completion(
        messages = "No input example has been defined for this model task."
    )
