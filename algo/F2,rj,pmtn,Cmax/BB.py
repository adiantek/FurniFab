import sys
import copy
import time as clock
### wrzucić na githuba
the_best = sys.float_info.max
time_the_best = []
time2_the_best = []
order_the_best=[]
stop_recursion = False
def add_to_the_first_machine(x,time):
    if len(time)==0:
        time.append(x[0])
    elif x[0] > time[-1]:
        time.append(x[0])
    else:
        time.append(time[-1])
    return time

def add_to_the_second_machine(x, time, time2):
    if len(time2) == 0:
        #time2.pop(-1)
        time2.append(time[-1])
        time2.append(time2[-1]+x[2])
    else:
        if time2[-1] > time[-1]:
            time2.append(time2[-1])
            time2.append(time2[-1] + x[2])
        else:
            time2.append(time[-1])
            time2.append(time[-1] + x[2])
    return time2

def update_parameters(time2,time,order):
    global the_best,order_the_best, time_the_best, time2_the_best
    the_best = time2[-1]
    order_the_best = order.copy()
    time_the_best = time.copy()
    time2_the_best = time2.copy()

def do_whole_task(time,tasks,updated_tasks, x, sorted_data, time2, order, index, n, timeout_seconds):
    time.append(time[-1] + x[1])
    current_task_index = sorted_data[x[0]].index(x)
    sorted_data[x[0]].pop(current_task_index)
    updated_tasks_copy = [z for z in updated_tasks if z[3] != x[3]]
    time2 = add_to_the_second_machine(x, time.copy(), time2.copy())
    if len(sorted_data[x[0]]) == 0:
        del sorted_data[x[0]]
    filtered_tasks = [t for t in updated_tasks_copy if t[0] <= time[-1] and t[1] > 0] ### następnie zadanie trzeba wybrać spośród poprzednich, nieskończonych zadań
    if len(filtered_tasks) > 0:
        for f in filtered_tasks:
            BB(f, copy.deepcopy(sorted_data), time.copy(), time2.copy(), order.copy(), updated_tasks_copy, n, timeout_seconds)
    else: ### jeśli nie ma poprzednich nieskończonych zadań, bierzemy następne zadanie, które dopiero przyjdzie
        for i in range(len((sorted_data[tasks[index]]))):
            next_task = sorted_data[tasks[index]][i]
            BB(next_task, copy.deepcopy(sorted_data), time.copy(), time2.copy(), order.copy(), updated_tasks_copy, n, timeout_seconds)

def abort_task(time,tasks,updated_tasks, x, sorted_data, time2, order, index, timeout_seconds):
    t = tasks[index] - time[-1]  ### wyliczanie ile czasu obecne zadanie będzie wykonywane zanim pojawi się następne
    time.append(time[-1] + t)
    updated_tasks_copy = [z if z[3] != x[3] else z[:1] + (z[1] - t,) + z[2:] for z in updated_tasks]
    x_old = x
    x = x[:1] + (x[1] - t,) + x[2:]
    current_task_index = sorted_data[x[0]].index(x_old)
    sorted_data[x[0]][current_task_index] = x
    for i in range(len((sorted_data[tasks[index]]))):
        next_task = sorted_data[tasks[index]][i]
        BB(next_task, copy.deepcopy(sorted_data), time.copy(), time2.copy(), order.copy(), updated_tasks_copy , timeout_seconds = timeout_seconds)

    BB(x, copy.deepcopy(sorted_data), time.copy(), time2.copy(), order.copy(), updated_tasks_copy,  timeout_seconds = timeout_seconds) #### jednak nie przerywamy zadania
### n to parametr określający czy będą jeszcze napływać nowe zadania
def BB(x,sorted_data, time, time2, order, updated_tasks, n = True, timeout_seconds = None):
    global the_best
    global order_the_best
    global time_the_best
    global time2_the_best
    global start_time
    global stop_recursion
   
    if stop_recursion:
        return
    order.append(x[3])

    time = add_to_the_first_machine(x, time.copy())
    if len(updated_tasks)==1 and updated_tasks[0]==x: ### zostało już tylko jedno zadanie do wykonania
        time.append(time[-1]+x[1])
        time2 = add_to_the_second_machine(x, time.copy(), time2.copy())

        if time2[-1]<the_best:
            update_parameters(time2.copy(), time.copy(),order)
        if timeout_seconds != None and clock.time() - start_time > timeout_seconds:
            stop_recursion = True
        return
    if len(time2) == 0 or time2[-1] < the_best:
        tasks = list(sorted_data.keys())
        next_task = None
        for element in tasks: ### ustalenie indeksu następnego zadania, jest to kluczowe w celu sprawdzenia jego czasu gotowości
            if element > time[-1]:
                next_task = element
                break
        if next_task != None:
            index = tasks.index(next_task)
        else:
            index = None
        if index and n == True:
            if tasks[index] < time[-1] + x[1]: ### trzeba uwzględnić fakt, że zadanie może zostać przerwane
                abort_task(time.copy(), tasks.copy(), updated_tasks.copy(), x, copy.deepcopy(sorted_data), time2.copy(), order.copy(), index, timeout_seconds = timeout_seconds)
            else: ### zadanie wykona się w całości, nie przybędzie w tym czasie żadne nowe zadanie
                do_whole_task(time.copy(), tasks.copy(), updated_tasks.copy(), x, copy.deepcopy(sorted_data), time2.copy(), order.copy(), index, n, timeout_seconds = timeout_seconds)
        elif index == None and n == True: #### ostatnie zadanie na liście, nie będą przychodzić już następne zadania
            n=False
            do_whole_task(time.copy(), tasks.copy(), updated_tasks.copy(), x, copy.deepcopy(sorted_data), time2.copy(), order.copy(), index, n, timeout_seconds = timeout_seconds)
        else: ### poruszamy się po poprzednich nieskończonych zadaniach
            do_whole_task(time.copy(), tasks.copy(), updated_tasks.copy(), x, copy.deepcopy(sorted_data), time2.copy(), order.copy(), index, n, timeout_seconds = timeout_seconds)




def prepare_data():
    global order_the_best, time_the_best, order_list, time2_the_best, result, result2

    for index, value in enumerate(order_the_best):
        time_start_index = index * 2
        time_end_index = time_start_index + 1

        if time_end_index < len(time_the_best):
            time = [time_the_best[time_start_index], time_the_best[time_end_index]]
            if value in result:
                result[value].append(time)
            else:
                result[value] = [time]

    order_list = []

    for item in reversed(order_the_best):
        if item not in order_list:
            order_list.append(item)
    order_list = reversed(order_list)
    for index, value in enumerate(order_list):
        time_start_index = index * 2
        time_end_index = time_start_index + 1

        if time_end_index < len(time_the_best):
            time = [time2_the_best[time_start_index], time2_the_best[time_end_index]]
            result2[value] = [time]

    result = {k: result[k] for k in sorted(result)}
    result2 = {k: result2[k] for k in sorted(result2)}


##### tylko do frontendu
def run_algorithm(input_data):
    data = {}
    tasks = []
    k=1
    for numbers in input_data:
        element = tuple(numbers) + (k,)
        if int(element[0]) in data:
            data[int(element[0])].append(element)
        else:
            data[int(element[0])] = [element]
        tasks.append(element)
        k = k + 1


    sorted_data = {k: data[k] for k in sorted(data)}
    sorted_tasks = sorted(tasks, key=lambda x: x[0])

    for i in range(len(sorted_data[list(sorted_data.keys())[0]])):
        for j in range(len(sorted_data[list(sorted_data.keys())[1]])):
            BB(sorted_data[list(sorted_data.keys())[0]][i], copy.deepcopy(sorted_data), [], [], [], sorted_tasks)
    prepare_data()
    res = {}
    res['result_1'] = result
    res['result_2'] = result2
    return res
    
if __name__ == "__main__":
    # print("Ilość wprowadzonych danych:")
    # n = int(input())
    data = {}
    tasks = []
    k = 1
    print("Nazwa pliku:")
    file_name = input()
    
    with open(file_name, 'r') as file:
        for numbers in file:
            element = tuple(map(int, numbers.split())) + (k,)
            if int(element[0]) in data:
                data[int(element[0])].append(element)
            else:
                data[int(element[0])] = [element]
            tasks.append(element)
            k = k + 1
    sorted_data = {k: data[k] for k in sorted(data)}
    sorted_tasks = sorted(tasks, key=lambda x: x[0])
    
    for i in range(len(sorted_data[list(sorted_data.keys())[0]])):
        for j in range(len(sorted_data[list(sorted_data.keys())[1]])):
            BB(sorted_data[list(sorted_data.keys())[0]][i], copy.deepcopy(sorted_data), [], [], [], sorted_tasks)
    prepare_data()
    print("----------------------")
    print(the_best)
    print(order_the_best)
    print(time_the_best)
    print(time2_the_best)
    
    print(result)
    print(result2)
    with open('result_BB.txt', 'w') as file:
        file.write("Zadania na pierwszej maszynie: \n")
        for key, value in result.items():
            file.write(f'{key}: {value}\n')
        file.write("Zadania na drugiej maszynie: \n")
        for key, value in result2.items():
            file.write(f'{key}: {value}\n')
