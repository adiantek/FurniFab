import random
from memory_profiler import profile, memory_usage
A = set()
schedule = {}
schedule2 = {}
t_second_machine = 0



def add_to_schedule(task, start, duration):
    global schedule,t_second_machine
    task_id = task[3]
    schedule[task_id].append([start, start + duration])
    if t_second_machine < start + duration:
        t_second_machine = start+duration



def add_to_schedule2(task, start, duration):
    global t_second_machine
    global schedule2
    task_id = task[3]

    schedule2[task_id].append([t_second_machine, t_second_machine + duration])
    t_second_machine = t_second_machine + duration


def JohnsonRule(tasks):
    A = [x for x in tasks if x[1] <= x[2]]
    A.sort(key=lambda x: x[1])
    B = []
    if len(A) == 0:
        B = [x for x in tasks if x[1] > x[2]]
        B.sort(key=lambda x: x[2], reverse=True)
    con_AB = A + B
    return con_AB[0]


def random_task(tasks):
    tasks_list = list(tasks)
    random_element = random.choice(tasks_list)
    return random_element


def min_t1(tasks):
    min_t = min(tasks, key=lambda x: x[1])

    return min_t


def max_t1(tasks):
    max_t = max(tasks, key=lambda x: x[1])

    return max_t


def max_t2(tasks):
    max_t = max(tasks, key=lambda x: x[2])

    return max_t


def min_t1DIVt2(tasks):
    min_t = min(tasks, key=lambda x: x[1] / x[2])

    return min_t

def assess_weight(tasks):
    max_w = max(tasks, key=lambda x: (-1 * x[1]) + x[2])
    return max_w
def Horn(J):
    global t_second_machine
    t1 = min(task[0] for task in J)

    A = set()
    while J or A:

        A |= {task for task in J if task[0] <= t1}
        J -= A

        if not J:
            t2 = float('inf')
        else:
            t2 = min(task[0] for task in J)

        if A:
            task = JohnsonRule(A)#assess_weight(A)
            l = min(task[1], t2 - t1)
            add_to_schedule(task, t1, l)

            if task[1] == l:
                A.remove(task)
                add_to_schedule2(task, l + t1, task[2])
            else:

                A = {(r, t - l, d, n) if n == task[3] else (r, t, d, n) for (r, t, d, n) in A}

            t1 += l
        else:
            t1 = t2


def run_algorithm(input_data):
    global schedule, schedule2, A, t_second_machine
    schedule, schedule2 = {}, {}
    t_second_machine = 0
    A = set()
    k = 1
    tasks = []
    for numbers in input_data:
        element = tuple(numbers) + (k,)
        tasks.append(element)
        schedule[k] = []
        schedule2[k] = []
        k = k + 1
    tasks.sort(key=lambda x: x[0])

    J = set(tasks)
    Horn(J)
    res = {}
    res['result_1'] = schedule
    res['result_2'] = schedule2
    res['c_max'] = t_second_machine
    return res


if __name__ == '__main__':
    print("Nazwa pliku:")
    file_name = input()
    k = 1
    tasks = []
    with open(file_name, 'r') as file:
        for numbers in file:
            element = tuple(map(int, numbers.split())) + (k,)
            tasks.append(element)
            schedule[k] = []
            schedule2[k] = []
            k = k + 1

    tasks.sort(key=lambda x: x[0])

    J = set(tasks)
    Horn(J)
    print(schedule)
    print(schedule2)
    print(t_second_machine)