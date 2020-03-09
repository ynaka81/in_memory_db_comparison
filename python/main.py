import random
from concurrent.futures import ThreadPoolExecutor

import grpc
from fasteners import ReaderWriterLock
from google.protobuf import empty_pb2

import db_pb2
import db_pb2_grpc


class DB(db_pb2_grpc.DBServicer):

    N = int(1e6)

    def __init__(self):
        self.lock = ReaderWriterLock()
        self.records = [random.randint(0, DB.N / 10) for _ in range(DB.N)]

    def Search(self, request, context):
        with self.lock.read_lock():
            records = [
                db_pb2.Record(index=index, value=value)
                for index, value in enumerate(self.records)
                if value == request.value
            ]
        return db_pb2.Records(records=records)

    def Add(self, request, context):
        with self.lock.write_lock():
            self.records.append(request.value)
        return empty_pb2.Empty()

    def Update(self, request, context):
        with self.lock.write_lock():
            for record in request.records:
                self.records[record.index] = record.value
        return empty_pb2.Empty()


def serve():
    server = grpc.server(ThreadPoolExecutor(max_workers=10))
    db_pb2_grpc.add_DBServicer_to_server(DB(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    print('start server.')
    server.wait_for_termination()


if __name__ == '__main__':
    serve()
