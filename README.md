# Description
Benchmarking [FastAPI](https://fastapi.tiangolo.com/) (Python), [Flask](https://flask.palletsprojects.com/en/2.2.x/) (Python) and [Actix-web](https://actix.rs/) (Rust) with an REST API to return the contents of a .csv file.

## Running the servers
>Create a Python `venv` and install libraries in [requirements.txt](requirements.txt).

### Actix-web
    ./actix

### FastAPI
    uvicorn main-fastapi:app --port 8080

### Flask
    python main-flask.py

## Benchmarking (one server at a time)
>Requires `siege`.

Use case: 10 users hitting http://127.0.0.1:8080/dresses for 30s with 1s delay between each call.

    siege -c 10 -t30s -d 1 http://127.0.0.1:8080/dresses

## Results


| Metric                 | Actix-web    | FastAPI |Flask|
|------------------------|---------|--------|-----|
| Transactions           | 546     | 73     |343|
| Availability           | 100.00  | 100.00 |100.00|
| Elapsed Time           | 29.47   | 29.00  |29.83|
| Data Transferred       | 1293.39 | 283.33 |1331.25|
| Response Time          | 0.03    | 3.34   |0.35|
| Transaction Rate       | 18.53   | 2.52   |11.50|
| Throughput             | 43.89   | 9.77   |44.63|
| Concurrency            | 0.58    | 8.40   |4.06|
| Successful Transactions | 546     | 73     |343|
| Failed Transactions    | 0       | 0      |0|
| Longest Transaction    | 0.10    | 5.14   |1.14|
| Shortest Transaction   | 0.02    | 2.36   |0.08|

