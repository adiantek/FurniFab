import sys
data=[]
time2 = {}
time = {}
order = []
def add_task_to_second_machine(task,time_last_task2,T,L):
    global time2
    if time_last_task2 > T + L:
        time2[task[3]].append([time_last_task2, time_last_task2 + task[2]])
        time_last_task2 = time_last_task2 + task[2]
    else:
        time2[task[3]].append([T + L, T + L + task[2]])
        time_last_task2 = T + L + task[2]
    return time_last_task2
def MainLoop():
    global time2, data, order, time
    time_last_task2 = 0
    T = min(x[0] for x in data)
    while True:
        E = [x for x in data if x[0] <= T ]
        a = [element for element in data if element[0] > T]
        if len(a)>0:
            T2 =  min(x[0] for x in a)
        else:
            T2=sys.float_info.max
        if len(E)>0:
            task = min(E, key=lambda x: x[1])
            L = min(task[1],T2-T)
            time[task[3]].append([T,T+L])
            if task[1]<=L:
                time_last_task2 = add_task_to_second_machine(task,time_last_task2,T,L)
                data = [x for x in data if x[3] != task[3]]
            else:
                data = [x if x[3] != task[3] else x[:1] + (x[1] - L,) + x[2:] for x in data]
            order.append(task[3])
        else:
            L = T2 - T
        if len(data)==0:
            break
        else:
            T=T+L
print("Nazwa pliku:")
file_name = input()
k=1
with open(file_name, 'r') as file:
    for numbers in file:
        element = tuple(map(int, numbers.split())) + (k,)
        data.append(element)
        time[k]=[]
        time2[k]=[]
        k=k+1
MainLoop()
print(order)
print(time)
print(time2)
with open('result_pa.txt', 'w') as file:
    # Iteracja przez słownik i zapisywanie kluczy i wartości
    file.write("Zadania na pierwszej maszynie: \n")
    for key, value in time.items():
        file.write(f'{key}: {value}\n')
    file.write("Zadania na drugiej maszynie: \n")
    for key, value in time2.items():
        file.write(f'{key}: {value}\n')