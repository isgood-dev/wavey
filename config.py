import json
from typing import Any

def write(key: str, value: Any):
    with open("config.json") as file:
        decoded = json.load(file)
        
    decoded[key] = value

    with open("config.json", "w") as file:
        json.dump(decoded, file, indent=4)

def view(key: str):
    with open("config.json") as file:
        decoded = json.load(file)

    return decoded[key]