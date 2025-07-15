from llama_cpp import Llama

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

if __name__ == "__main__":
    run_local_model()
