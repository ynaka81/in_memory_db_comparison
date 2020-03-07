import logging
import time
from concurrent.futures import ThreadPoolExecutor

import click
import grpc
import random
from pythonjsonlogger import jsonlogger

import db_pb2
import db_pb2_grpc


def send_request(old, new):
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = db_pb2_grpc.DBStub(channel)
        # Search records.
        start = time.time()
        response = stub.Search(db_pb2.Value(value=old))
        logger.info('receive request.', extra={'type': 'search', 'elapsed': time.time() - start})
        # Update records value.
        records = response.records
        for record in records:
            record.value = new
        start = time.time()
        stub.Update(db_pb2.Records(records=records))
        logger.info('receive request.', extra={'type': 'add', 'elapsed': time.time() - start})


@click.command()
@click.option('-n', '--name', required=True, help='Experiment name.')
@click.option('-p', '--parallel', default=1, help='Number of parallel requests.')
def run(name, parallel):
    # Prepare file handler.
    file_handler = logging.FileHandler(f'results/{name}_{parallel}')
    file_handler.setFormatter(formatter)
    logger.addHandler(file_handler)
    # Start to measure performance.
    N = int(1e6)
    with ThreadPoolExecutor(max_workers=parallel) as executor:
        for i in range(parallel * 10):
            old = random.randint(0, N)
            new = random.randint(0, N)
            executor.submit(send_request, old, new)


if __name__ == '__main__':
    # Setup logging.
    logger = logging.getLogger()
    formatter = jsonlogger.JsonFormatter('(levelname) (asctime) (pathname) (lineno) (message)')
    handler = logging.StreamHandler()
    handler.setFormatter(formatter)
    logger.addHandler(handler)
    logger.setLevel(logging.INFO)
    # Run command.
    run()
