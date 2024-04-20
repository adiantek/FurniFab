import random
schedule = {}
schedule2 = {}
A = set()
t_second_machine = 0


def add_to_schedule(task, start, duration):
    global schedule
    task_id = task[3]
    schedule[task_id].append([start, start + duration])


def add_to_schedule2(task, start, duration):
    global t_second_machine
    global schedule2
    task_id = task[3]
    if t_second_machine == 0:
        schedule2[task_id].append([start, t_second_machine + duration])
    else:
        schedule2[task_id].append([t_second_machine, t_second_machine + duration])
    if len(schedule2)==0 or t_second_machine < start:
        t_second_machine = start + duration
    else:
        t_second_machine = t_second_machine + duration
def C_max(tasks):
    max_order = 0
    current_order=0
    for i in range (len(tasks)):
        current_order = current_order + tasks[i][1]
        max_order = max(max_order, current_order)
        max_order = max_order + tasks[i][2]
    return max_order
def swap_two_elements(tasks):
    tasks_len = len(tasks)
    indexes = random.sample(range(0, tasks_len), 2)
    tasks_1 = tasks[indexes[0]]
    tasks_2 = tasks[indexes[1]]
    tasks[indexes[0]] = tasks_2
    tasks[indexes[1]] = tasks_1
    return tasks
def move_the_element(tasks):
    tasks_len = len(tasks)
    move_index = random.randint(0, tasks_len - 1)
    element = tasks.pop(move_index)
    new_index = random.randint(0, tasks_len - 2)
    tasks.insert(new_index, element)
    return tasks
def shift_in_cycle(tasks):
    new_tasks= tasks[-1:] + tasks[:-1]
    return new_tasks
def reverse_the_subsequence(tasks):
    tasks_len = len(tasks)
    index_start, index_end = sorted(random.sample(range(tasks_len), 2))
    subsequence = tasks[index_start:index_end + 1]
    inverted_subsequence = subsequence[::-1]
    new_tasks = tasks[:index_start] + inverted_subsequence + tasks[index_end+ 1:]
    return new_tasks
def shuffle_the_subsequence(tasks):
    tasks_len = len(tasks)
    index_start, index_end = sorted(random.sample(range(tasks_len), 2))
    subsequence = tasks[index_start:index_end + 1]
    random.shuffle(subsequence)
    new_tasks = tasks[:index_start] + subsequence + tasks[index_end+ 1:]
    return new_tasks
def move_the_subsequence(tasks):
    subseqence_len = random.randint(2,len(tasks)//2)
    index_start = random.randint(0, len(tasks) - subseqence_len - 1)
    subsequence = tasks[index_start:index_start + subseqence_len]
    new_sequence = tasks[:index_start] + tasks[index_start + subseqence_len:]
    new_index = random.randint(0, len(tasks) - subseqence_len - 1)
    new_sequence = new_sequence[:new_index] + subsequence + new_sequence[new_index:]
    return new_sequence
def block_swap(tasks):
    n = len(tasks)
    block_len = 3
    index_block1_start = random.randint(0, n - 2 * block_len)
    index_block2_start = random.randint(index_block1_start + block_len, n - block_len)
    block1 = tasks[index_block1_start:index_block1_start + block_len]
    block2 = tasks[index_block2_start:index_block2_start + block_len]
    new_sequence = tasks[:index_block1_start] + block2 + tasks[index_block1_start + block_len:index_block2_start] + block1 + tasks[index_block2_start + block_len:]
    return new_sequence
def perturbation(tasks):
    number = max(3, len(tasks)//10)
    for i in range(number):
        idx1, idx2 = random.sample(range(len(tasks)), 2)
        tasks[idx1], tasks[idx2] = tasks[idx2], tasks[idx1]
    return tasks
def local_search(tasks, function):
    the_best_sol = tasks.copy()
    result = tasks.copy()
    the_best_score = C_max(tasks)
    function = globals()[function]
    no_improvements_iter = 0
    while True:
        result = function(result)
        current_len = C_max(result)
        if current_len < the_best_score:
            the_best_sol = result
            the_best_score = current_len
            no_improvements_iter = 0
        else:
            no_improvements_iter +=1
        if no_improvements_iter == len(tasks):
            result = perturbation(result)
        if no_improvements_iter == len(tasks)*2:
            break
    return the_best_sol


def NeighborhoodChange(solution1, solution2, k):
    if C_max(solution2)<C_max(solution1):
        solution1 = solution2
        k=0
    else:
        k=k+1
    return solution1, k
def VNS(x):
    neighborhood = ['swap_two_elements','move_the_element', 'shift_in_cycle', 'reverse_the_subsequence', 'shuffle_the_subsequence', 'move_the_subsequence', 'block_swap']
    k=0
    while k<len(neighborhood):
        solution = local_search(x, neighborhood[k])
        x, k = NeighborhoodChange(x,solution,k)
    return x


def currentTask(min_elements,list):
    for l in list:
        for m in min_elements:
            if m[3]==l[3]:
                return m
def Horn(J, list):
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
            task =currentTask(A, list)#JohnsonRule(A)
            l = min(task[1], t2 - t1)
            add_to_schedule(task, t1, l)


            if task[1] <= l:
                A.remove(task)
            else:

                A = {(r, t - l, d, n) if n == task[3] else (r, t, d, n) for (r, t, d, n) in A}
            if task[1]==l:
                add_to_schedule2(task, l+t1, task[2])
            t1 += l
        else:
            t1 = t2



def MainLoop(tasks):
    random.shuffle(tasks)
    no_improvements = 0
    the_best_score = C_max(tasks)
    the_best_order = tasks.copy()
    for _ in range(len(tasks)):
        random.shuffle(tasks)
        x = VNS(tasks)
        score = C_max(x)
        if score<the_best_score:
            the_best_score = score
            the_best_order = x
            no_improvements = 0
            tasks = x
        else:
            no_improvements +=1
        if no_improvements == len(tasks)// 5:
            tasks = perturbation(the_best_order)
        if no_improvements == len(tasks)//2:
            break
    return the_best_order
def run_algorithm(input_data):
    global schedule, schedule2
    data = []
    k=1
    for numbers in input_data:
        element = tuple(numbers) + (k,) #tuple(map(int, numbers.split())) + (k,)
        data.append(element)
        k=k+1
    the_best_order = MainLoop(data)
    Horn(set(the_best_order), data)
    res = {}
    res['result_1'] = schedule
    res['result_2'] = schedule2
    return res

tasks=[]
k=1
print("Nazwa pliku:")
file_name = input()
r = []
with open(file_name, 'r') as file:
    for numbers in file:
        r.append(numbers)
        element = tuple(map(int, numbers.split())) + (k,)
        tasks.append(element)
        schedule[k] = []
        schedule2[k] = []
        k = k + 1
print(run_algorithm(r))