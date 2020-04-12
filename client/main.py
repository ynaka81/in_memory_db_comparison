import logging
import sys
import time
from concurrent.futures import ThreadPoolExecutor

import click
import grpc
import random
from pythonjsonlogger import jsonlogger

import db_pb2
import db_pb2_grpc


def test_add(query, value, cycle):
    for _ in range(cycle):
        with grpc.insecure_channel('localhost:50051') as channel:
            stub = db_pb2_grpc.DBStub(channel)
            # Search records.
            start = time.time()
            stub.Search(db_pb2.Value(value=query))
            logger.info('receive request.', extra={'type': 'read', 'elapsed': time.time() - start})
            # Add a record.
            start = time.time()
            stub.Add(db_pb2.Values(values=[value]))
            logger.info('receive request.', extra={'type': 'write', 'elapsed': time.time() - start})


def test_update(query, value, cycle):
    for _ in range(cycle):
        with grpc.insecure_channel('localhost:50051') as channel:
            stub = db_pb2_grpc.DBStub(channel)
            # Search records.
            start = time.time()
            response = stub.Search(db_pb2.Value(value=query))
            logger.info('receive request.', extra={'type': 'read', 'elapsed': time.time() - start})
            # Update records value.
            records = response.records
            for record in records:
                record.value = value
            start = time.time()
            stub.Update(db_pb2.Records(records=records))
            logger.info('receive request.', extra={'type': 'write', 'elapsed': time.time() - start})


def test_delete(query, value, cycle):
    for _ in range(cycle):
        with grpc.insecure_channel('localhost:50051') as channel:
            stub = db_pb2_grpc.DBStub(channel)
            # Search records.
            start = time.time()
            response = stub.Search(db_pb2.Value(value=query))
            logger.info('receive request.', extra={'type': 'read', 'elapsed': time.time() - start})
            # # Add records.
            values = db_pb2.Values(values=[r.value for r in response.records])
            values = db_pb2.Indexes(indexes=[r.index for r in response.records])
            start = time.time()
            stub.Add(values)
            logger.info('receive request.', extra={'type': 'write', 'elapsed': time.time() - start})
            # Delete searched records.
            indexes = db_pb2.Indexes(indexes=[r.index for r in response.records])
            stub.Delete(indexes)


test_methods = {
    'add': test_add,
    'update': test_update,
    'delete': test_delete
}


@click.command()
@click.option('-n', '--name', required=True, help='Experiment name.')
@click.option('-m', '--method', type=click.Choice(test_methods.keys()), required=True, help='Experiment name.')
@click.option('-p', '--parallel', default=1, help='Number of parallel requests.')
@click.option('-c', '--cycle', default=100, help='Number of test cycles.')
def run(name, method, parallel, cycle):
    # Prepare file handler.
    file_handler = logging.FileHandler(f'results/{name}_{parallel}')
    file_handler.setFormatter(formatter)
    logger.addHandler(file_handler)
    # Start to measure performance.
    N = int(1e6)
    with ThreadPoolExecutor(max_workers=parallel) as executor:
        for i in range(parallel):
            query = random.randint(0, N)
            value = random.randint(0, N)
            executor.submit(test_methods[method], query, value, cycle)


if __name__ == '__main__':
    # Setup logging.
    logger = logging.getLogger()
    formatter = jsonlogger.JsonFormatter('(levelname) (asctime) (pathname) (lineno) (message)')
    handler = logging.StreamHandler(sys.stdout)
    handler.setFormatter(formatter)
    logger.addHandler(handler)
    logger.setLevel(logging.INFO)
    # Run command.
    run()
