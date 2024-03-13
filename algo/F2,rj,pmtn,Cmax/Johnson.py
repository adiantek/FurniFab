from collections import defaultdict
time = []
time2 = []
order = []
all_tasks=[]
result = defaultdict(list)
result2 = defaultdict(list)
def JohnsonRule(tasks):
    A = [x for x in tasks if x[1] <= x[2]]
    B = [x for x in tasks if x[1] > x[2]]
    A.sort(key=lambda x: x[1])
    B.sort(key=lambda x: x[2], reverse=True)
    con_AB = A+B
    return con_AB[0]
def add_task_to_second_machine(current_task):
    global time2,time
    if len(time2)==0:
        time2.append(time[-1])
        time2.append(time[-1]+current_task[2])
    else:
        if time2[-1]>time[-1]:
            time2.append(time2[-1])
            time2.append(current_task[2]+time2[-1])
        else:
            time2.append(time[-1])
            time2.append(current_task[2] + time[-1])
    result2[current_task[3]].append([time2[-2],time2[-1]])
def add_the_task_to_the_first_machine(current_task, next_task):
    global time, all_tasks,order
    if len(time)==0:
        order.append(current_task[3])
        time.append(current_task[0])
    elif time[-1]>current_task[0]:
        order.append(current_task[3])
        time.append(time[-1])
    else:
        order.append(current_task[3])
        time.append(current_task[0])
    if time[-1]+current_task[1]>next_task:
        t = next_task-time[-1]
        all_tasks = [x if x[3] != current_task[3] else x[:1] + (x[1] - t,) + x[2:] for x in all_tasks]
        time.append(next_task)
        result[current_task[3]].append([time[-2],time[-1]])
    else:
        all_tasks = [x for x in all_tasks if x[3] != current_task[3]]
        time.append(time[-1]+current_task[1])
        result[current_task[3]].append([time[-2], time[-1]])
        add_task_to_second_machine(current_task)
def add_the_task_to_the_first_machine2(current_task):
    global time, all_tasks
    if len(time)==0:
        time.append(current_task[0])
    elif time[-1] > current_task[0]:
        time.append(time[-1])
    else:
        time.append(current_task[0])
    all_tasks = [x for x in all_tasks if x[3] != current_task[3]]
    time.append(time[-1]+current_task[1])
    result[current_task[3]].append([time[-2], time[-1]])
    order.append(current_task[3])
    add_task_to_second_machine(current_task)
def MainLoop():
    global time
    min_value = min(x[0] for x in all_tasks)
    min_elements = [x for x in all_tasks if x[0] == min_value]
    while True:
        if len(min_elements)>0:
            current_task = JohnsonRule(min_elements)
        else :
            current_task = JohnsonRule(all_tasks)
        if len(time)>0:
            updated_task = [x for x in all_tasks if x[0] > current_task[0] and x[0]>time[-1]]
        else:
            updated_task = [x for x in all_tasks if x[0] > current_task[0] ]
        if len(updated_task)>0:
            min_value = min(x[0] for x in updated_task)
            add_the_task_to_the_first_machine(current_task, min_value)
            min_elements = [x for x in all_tasks if x[0] <= min_value]
            if time[-1]<min_value and len([x for x in all_tasks if x[0] < time[-1]])>0:
                min_elements = [x for x in all_tasks if x[0] < min_value]
        else:
            min_elements = []
            add_the_task_to_the_first_machine2(current_task)
        if len(all_tasks) == 0:
            break

def run_algorithm(input_data):
    global all_tasks, result, result2
    k=1
    for numbers in input_data:
        element = tuple(numbers) + (k,)
        all_tasks.append(element)
        k=k+1
        result[element[3]] = []
        result2[element[3]] = []
    MainLoop()
    res = {}
    res['result_1'] = result
    res['result_2'] = result2
    return res

import json

if __name__ == '__main__':
    print("Nazwa pliku:")
    file_name = input()
    input_data = []
    with open(file_name, 'r') as file:
        for numbers in file:
            input_data.append(list(map(int, numbers.split())))
    print(json.dumps(run_algorithm(input_data)))
    print(list(set(time)))
    print(order)
    print(result)
    print(result2)
    with open('result_pa.txt', 'w') as file:
        file.write("Zadania na pierwszej maszynie: \n")
        for key, value in result.items():
            file.write(f'{key}: {value}\n')
        file.write("Zadania na drugiej maszynie: \n")
        for key, value in result2.items():
            file.write(f'{key}: {value}\n')