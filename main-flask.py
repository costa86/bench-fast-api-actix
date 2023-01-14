from flask import Flask
import csv

app = Flask(__name__)


@app.route("/dresses")
def read_dresses() -> list:
    with open("dress.csv", mode="r") as csv_file:
        return [i for i in csv.DictReader(csv_file)]

if __name__ == "__main__":
    app.run(port=8080)
