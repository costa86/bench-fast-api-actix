# Description
Benchmarking [FastAPI](https://fastapi.tiangolo.com/) (Python) and [Actix-web](https://actix.rs/) (Rust) with an REST API to return the contents of a .csv file.

## Running the servers

### Actix-web
    ./actix

### FastAPI
>Create a Python `venv` and install libraries in [requirements.txt](requirements.txt).

    uvicorn main:app --port 8080

## Benchmarking (one server at a time)
>Requires `siege`.

Use case: 10 users hitting http://127.0.0.1:8080/dresses for 30s with 1s delay between each call.

    siege -c 10 -t30s -d 1 http://127.0.0.1:8080/dresses

## Results


| Metric                 | Actix-web    | FastAPI |
|------------------------|---------|--------|
| Transactions           | 546     | 73     |
| Availability           | 100.00  | 100.00 |
| Elapsed Time           | 29.47   | 29.00  |
| Data Transferred       | 1293.39 | 283.33 |
| Response Time          | 0.03    | 3.34   |
| Transaction Rate       | 18.53   | 2.52   |
| Throughput             | 43.89   | 9.77   |
| Concurrency            | 0.58    | 8.40   |
| Successful Transactions | 546     | 73     |
| Failed Transactions    | 0       | 0      |
| Longest Transaction    | 0.10    | 5.14   |
| Shortest Transaction   | 0.02    | 2.36   |
