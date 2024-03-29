import random

A = set()
schedule = {}
schedule2 = {}
t_second_machine = 0



def add_to_schedule(task, start, duration):
    global schedule
    task_id = task[3]
    schedule[task_id].append([start, start + duration])


def add_to_schedule2(task, start, duration):
    global t_second_machine
    global schedule2
    task_id = task[3]
    schedule2[task_id].append([t_second_machine, t_second_machine + duration])
    if len(schedule2)==0 or t_second_machine < start:
        t_second_machine = start + duration
    else:
        t_second_machine = t_second_machine + duration



def JohnsonRule(tasks):
    A = [x for x in tasks if x[1] <= x[2]]
    A.sort(key=lambda x: x[1])
    B=[]
    if len(A)==0:
        B = [x for x in tasks if x[1] > x[2]]
        B.sort(key=lambda x: x[2], reverse=True)
    con_AB = A+B
    return con_AB[0]

def random_task(tasks):
    tasks_list = list(tasks)
    random_element = random.choice(tasks_list)
    return random_element

def min_t1(tasks):
    min_t = min(tasks, key=lambda x: x[1])

    return min_t
def max_t2(tasks):
    max_t = max(tasks, key=lambda x: x[1])

    return max_t

def min_t1DIVt2(tasks):
    min_t = min(tasks, key=lambda x: x[1]/x[2])
    return min_t

def Horn(J):

    t1 = min(task[0] for task in J)
    global t_second_machine
    A = set()
    while J or A:

        A |= {task for task in J if task[0] <= t1}
        J -= A


        if not J:
            t2 = float('inf')
        else:
            t2 = min(task[0] for task in J)


        if A:
            task_with_min_d =JohnsonRule(A)
            l = min(task_with_min_d[1], t2 - t1)
            add_to_schedule(task_with_min_d, t1, l)


            if task_with_min_d[1] <= l:
                A.remove(task_with_min_d)
            else:

                A = {(r, t - l, d, n) if n == task_with_min_d[3] else (r, t, d, n) for (r, t, d, n) in A}
            if task_with_min_d[1]==l:
                add_to_schedule2(task_with_min_d, l+t1, task_with_min_d[2])
            t1 += l
        else:
            t1 = t2

# Wynik

print("Nazwa pliku:")
file_name = input()
k=1
tasks = []
with open(file_name, 'r') as file:
    for numbers in file:
        element = tuple(map(int, numbers.split())) + (k,)
        tasks.append(element)
        schedule[k] = []
        schedule2[k] = []
        k=k+1
tasks.sort(key=lambda x: x[0])

J = set(tasks)
Horn(J)
print(schedule)
print(schedule2)
