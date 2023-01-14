from fastapi import FastAPI
import csv

app = FastAPI()


@app.get("/dresses")
async def read_dresses() -> list:
    with open("dress.csv", mode="r") as csv_file:
        return [i for i in csv.DictReader(csv_file)]
