from flask import Flask
import csv

app = Flask(__name__)


@app.route("/dresses")
def read_dresses() -> list:
    dresses = []
    with open("dress.csv", mode="r") as csv_file:
        csv_reader = csv.DictReader(csv_file)
        for row in csv_reader:
            dresses.append(row)
    return dresses


if __name__ == "__main__":
    app.run(port=8080)
