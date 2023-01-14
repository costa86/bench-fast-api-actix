from fastapi import FastAPI
import csv

app = FastAPI()


@app.get("/dresses")
async def read_dresses() -> list:
    dresses = []
    with open("dress.csv", mode="r") as csv_file:
        csv_reader = csv.DictReader(csv_file)
        for row in csv_reader:
            dresses.append(row)
    return dresses
