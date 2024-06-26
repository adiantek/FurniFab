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
def C_max(tasks):
    max_order = 0
    current_order=0
    for i in range (len(tasks)):
        current_order = current_order + tasks[i][1]
        max_order = max(max_order, current_order)
        max_order = max_order + tasks[i][2]
    return max_order
def NEH(tasks, new_task):
    current_sol = []
    solution = []
    for k in range(len(tasks)+1):
        permutation_k =  tasks.copy()
        permutation_k.insert(k, new_task)
        current_sol.append(permutation_k)
    the_best_len = None
    for i in range(len(current_sol)):
        len_order = C_max(current_sol[i])
        if the_best_len == None or len_order<the_best_len:
            the_best_len = len_order
            solution = current_sol[i]
    return solution
def currentTask(min_elements,list):
    for l in list:
        for m in min_elements:
            if m[3]==l[3]:
                return m
def Horn(J):
    order = []
    global t_second_machine
    t1 = min(task[0] for task in J)

    A = set()
    k=0
    while J or A:
        k=k+1
        A |= {task for task in J if task[0] <= t1}
        J -= A


        if not J:
            t2 = float('inf')
        else:
            t2 = min(task[0] for task in J)


        if A:
            for a in A:
                if a not in order:
                    order =NEH(order, a)#JohnsonRule(A)
            task = order[0]
            l = min(task[1], t2 - t1)
            add_to_schedule(task, t1, l)



            if task[1]==l:
                A.remove(task)
                order.pop(0)
                add_to_schedule2(task, l+t1, task[2])
            else:
               # order2 = [(r, t - l, d, n) if n == task[3] else (r, t, d, n) for (r, t, d, n) in A]
                order.pop(0)
                order = NEH(order, (task[0], task[1]-l, task[2], task[3]))
                A = {(r, t - l, d, n) if n == task[3] else (r, t, d, n) for (r, t, d, n) in A}
            t1 += l
        else:
            t1 = t2
def run_algorithm(input_data):
    global schedule, schedule2, A, t_second_machine
    A = set()
    schedule = {}
    schedule2 = {}
    t_second_machine = 0
    data = []
    k=1
    for numbers in input_data:
        element = tuple(numbers) + (k,)
        data.append(element)
        schedule[k] = []
        schedule2[k] = []
        k=k+1
    data = sorted(data, key=lambda x: x[1]+x[2], reverse=True)
    Horn(set(data))
    res = {}
    res['result_1'] = schedule
    res['result_2'] = schedule2
    res['c_max'] = t_second_machine
    return res
if __name__ == '__main__':
    tasks=[]
    k=1
    print("Nazwa pliku:")
    file_name = input()
    r = []
    with open(file_name, 'r') as file:
        for numbers in file:
            element = tuple(map(int, numbers.split())) + (k,)
            r.append(numbers)
            tasks.append(element)
            schedule[k] = []
            schedule2[k] = []
            k = k + 1
    tasks = sorted(tasks, key=lambda x: x[1]+x[2], reverse=True)
    Horn(set(tasks))
    print("Rozwiązanie: ")
    print(schedule)
    print(schedule2)
    print(t_second_machine)
#print(run_algorithm(r))