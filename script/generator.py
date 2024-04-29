from argparse import ArgumentParser, Namespace
from json import dump
from random import randint, seed
from sys import stdout


def create_parser() -> ArgumentParser:
    """Creates the argument parser."""
    parser = ArgumentParser(description='Generates test data for FurniFab application')

    parser.add_argument('amount', type=int, help='Amount of tasks to generate')
    parser.add_argument('--seed', type=int, help='Seed for random number generator')
    parser.add_argument('--output', type=str, help='Output file. Default is stdout.')
    parser.add_argument('--prefix', default='Task ', type=str, help='Task name prefix')
    parser.add_argument('--time-min', default=10, type=int, help='Minimum task process time in minutes')
    parser.add_argument('--time-max', default=240, type=int, help='Maximum task process time in minutes')
    parser.add_argument('--weight-min', default=1, type=int, help='Minimum task weight in minutes')
    parser.add_argument('--weight-max', default=20, type=int, help='Maximum task weight in minutes')
    parser.add_argument('--conflict-min', default=0, type=int, help='Minimum task conflict amount')
    parser.add_argument('--conflict-max', default=5, type=int, help='Maximum task conflict amount')

    return parser


def generate_cutting_info(config: Namespace) -> dict[str, object]:
    """Generates cutting info for FurniFab application task."""
    return {
        'conflicts': [],
        'processTime': randint(config.time_min, config.time_max),
        'weight': randint(config.weight_min, config.weight_max),
    }


def generate_conflicts(config: Namespace, tasks: list[dict]):
    """Generates conflict info for FurniFab application task."""
    for identifier, task in enumerate(tasks):
        amount = randint(config.conflict_min, min(config.conflict_max, len(tasks) - 1))
        while len(task['cuttingInfo']['conflicts']) < amount:
            other_identifier = randint(0, config.amount - 1)

            if other_identifier == identifier:
                continue

            if other_identifier in task['cuttingInfo']['conflicts']:
                continue

            task['cuttingInfo']['conflicts'].append(other_identifier)
            tasks[other_identifier]['cuttingInfo']['conflicts'].append(identifier)


def generate_flow_info(config: Namespace) -> dict[str, object]:
    """Generates flow info for FurniFab application task."""
    return {
        'grindingProcessTime': randint(config.time_min // 2, config.time_max // 2),
        'lacqueringProcessTime': randint(config.time_min // 2, config.time_max // 2),
    }


def generate(config: Namespace) -> list[object]:
    """Generates test data for FurniFab application."""
    tasks = [{
        'id': identifier,
        'name': config.prefix + str(identifier),
        'cuttingInfo': generate_cutting_info(config),
        'flowInfo': generate_flow_info(config),
    } for identifier in range(config.amount)]

    generate_conflicts(config, tasks)

    return tasks


if __name__ == '__main__':
    parser = create_parser()
    args = parser.parse_args()

    if args.seed:
        seed(args.seed)

    tasks = generate(args)
    dump(tasks, open(args.output, 'w') if args.output else stdout)
