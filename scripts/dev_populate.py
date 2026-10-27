# /// script
# dependencies = [
#   "requests",
#   "tqdm",
#   "wonderwords",
# ]
# ///

from dataclasses import dataclass
import os
from tqdm import tqdm
import random
from wonderwords import RandomWord
import requests


BASE_URL = os.getenv("GUESTBOOK_URL", "http://localhost:8080")
NUM_ENTRIES = int(os.getenv("NUM_ENTRIES", "10"))


@dataclass
class EntryRequest:
    name: str
    email: str | None
    rating: int
    note: str


def is_alive() -> bool:
    resp = requests.get(f"{BASE_URL}/api/health")
    return resp.status_code == 200


def generate_entry() -> EntryRequest:
    r = RandomWord()
    return EntryRequest(
        name=r.word(),
        email=f"{r.word()}@{r.word()}.{r.word()}" if random.random() > 0.5 else None,
        rating=random.randint(0, 5),
        note=" ".join(r.random_words(random.randint(10, 40))),
    )


def submit_entry(entry: EntryRequest) -> None:
    resp = requests.post(f"{BASE_URL}/api/sign", json=vars(entry))
    resp.raise_for_status()


def main():
    for _ in tqdm(range(NUM_ENTRIES)):
        entry = generate_entry()
        submit_entry(entry)


if __name__ == "__main__":
    main()
