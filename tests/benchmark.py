from concurrent import futures
from fixtures import *  # noqa: F401,F403
from time import time, sleep
from tqdm import tqdm
from pyln.testing.utils import sync_blockheight
from pytest_benchmark.stats import Metadata
from contextlib import contextmanager

import logging
import pytest
import random
import sys
import os


num_workers = 75
num_payments = 10000


# To ensure optimal performance we need to run without the developer
# options.
assert os.environ.get("DEVELOPER", "0") == "0"


@contextmanager
def benchmark_throughput(benchmark, num_events: int):
    """Context manager to benchmark throughput.

    Repeated timed function calls measures latency, but throughput,
    with many parallel calls is better measured by dividing the time
    to completion, and divide it by the number of parallel calls.

    This results in a synthetic benchmark with no variance, but
    repeating can amend that.

    """
    m = Metadata(
        fixture=benchmark,
        iterations=num_events,
        options={
            "disable_gc": False,
            "timer": benchmark._timer,
            "min_rounds": 1,
            "max_time": benchmark._max_time,
            "min_time": benchmark._min_time,
            "warmup": False,
        },
    )
    benchmark._add_stats(m)
    benchmark._mode = "with benchmark_throughput(...)"
    start_time = time()
    yield
    m.update(time() - start_time)


@pytest.fixture
def executor():
    ex = futures.ThreadPoolExecutor(max_workers=num_workers)
    yield ex
    ex.shutdown(wait=False)


def test_single_hop(node_factory, executor):
    l1 = node_factory.get_node()
    l2 = node_factory.get_node()

    l1.rpc.connect(l2.rpc.getinfo()['id'], 'localhost:%d' % l2.port)
    l1.openchannel(l2, 4000000)

    print("Collecting invoices")
    fs = []
    invoices = []
    for i in tqdm(range(num_payments)):
        invoices.append(l2.rpc.invoice(1000, 'invoice-%d' % (i), 'desc')['payment_hash'])

    route = l1.rpc.getroute(l2.rpc.getinfo()['id'], 1000, 1)['route']
    print("Sending payments")
    start_time = time()

    def do_pay(i):
        p = l1.rpc.sendpay(route, i)
        r = l1.rpc.waitsendpay(p['payment_hash'])
        return r

    for i in invoices:
        fs.append(executor.submit(do_pay, i))

    for f in tqdm(futures.as_completed(fs), total=len(fs)):
        f.result()

    diff = time() - start_time
    print("Done. %d payments performed in %f seconds (%f payments per second)" % (num_payments, diff, num_payments / diff))


def test_single_payment(node_factory, benchmark):
    l1, l2 = node_factory.line_graph(2)

    def do_pay(l1, l2):
        invoice = l2.rpc.invoice(1000, 'invoice-{}'.format(random.random()), 'desc')['bolt11']
        l1.rpc.pay(invoice)

    benchmark(do_pay, l1, l2)


def test_forward_payment(node_factory, benchmark):
    l1, l2, l3 = node_factory.line_graph(3, wait_for_announce=True)

    def do_pay(src, dest):
        invoice = dest.rpc.invoice(1000, 'invoice-{}'.format(random.random()), 'desc')['bolt11']
        src.rpc.pay(invoice)

    benchmark(do_pay, l1, l3)


def test_long_forward_payment(node_factory, benchmark):
    nodes = node_factory.line_graph(21, wait_for_announce=True)

    def do_pay(src, dest):
        invoice = dest.rpc.invoice(1000, 'invoice-{}'.format(random.random()), 'desc')['bolt11']
        src.rpc.pay(invoice)

    benchmark(do_pay, nodes[0], nodes[-1])


def test_invoice(node_factory, benchmark):
    l1 = node_factory.get_node()

    def bench_invoice():
        l1.rpc.invoice(1000, 'invoice-{}'.format(time()), 'desc')

    benchmark(bench_invoice)


def test_pay(node_factory, benchmark):
    l1, l2 = node_factory.line_graph(2)

    invoices = []
    for _ in range(1, 100):
        invoice = l2.rpc.invoice(1000, 'invoice-{}'.format(random.random()), 'desc')['bolt11']
        invoices.append(invoice)

    def do_pay(l1, l2):
        l1.rpc.pay(invoices.pop())

    benchmark(do_pay, l1, l2)


def test_start(node_factory, benchmark):
    benchmark(node_factory.get_node)
