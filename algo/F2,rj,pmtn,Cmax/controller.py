from simpful import *
from collections import defaultdict
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

def create_FS(p1, p2, mean1, mean2):
    FS = FuzzySystem()


    M1_Very_Short = TrapezoidFuzzySet(0, 0, mean1 * 0.5, mean1 * 0.75, term="Very_Short")
    M1_Short = TriangleFuzzySet(mean1 * 0.5, mean1 * 0.75, mean1, term="Short")
    M1_Medium = TriangleFuzzySet(mean1 * 0.75, mean1, mean1 * 1.25, term="Medium")
    M1_Long = TriangleFuzzySet(mean1, mean1 * 1.25, mean1 * 1.5, term="Long")
    M1_Very_Long = TrapezoidFuzzySet(mean1 * 1.25, mean1 * 1.5, p1, p1, term="Very_Long")

    FS.add_linguistic_variable("M1", LinguisticVariable([M1_Very_Short, M1_Short, M1_Medium, M1_Long, M1_Very_Long], universe_of_discourse=[0, p1]))

    M2_Very_Short = TrapezoidFuzzySet(0, 0, mean2 * 0.5, mean2 * 0.75, term="Very_Short")
    M2_Short = TriangleFuzzySet(mean2 * 0.5, mean2 * 0.75, mean2, term="Short")
    M2_Medium = TriangleFuzzySet(mean2 * 0.75, mean2, mean2 * 1.25, term="Medium")
    M2_Long = TriangleFuzzySet(mean2, mean2 * 1.25, mean2 * 1.5, term="Long")
    M2_Very_Long = TrapezoidFuzzySet(mean2 * 1.25, mean2 * 1.5, p2, p2, term="Very_Long")

    FS.add_linguistic_variable("M2", LinguisticVariable([M2_Very_Short, M2_Short, M2_Medium, M2_Long, M2_Very_Long], universe_of_discourse=[0, p2]))


    P0 = TrapezoidFuzzySet(0, 0, 0.1, 0.2, term="very_low")
    P1 = TriangleFuzzySet(0.1, 0.3, 0.5, term="low")
    P2 = TriangleFuzzySet(0.4, 0.6, 0.8, term="medium")
    P3 = TriangleFuzzySet(0.7, 0.9, 1, term="high")
    P4 = TrapezoidFuzzySet(0.9, 1, 1, 1, term="very_high")

    FS.add_linguistic_variable("priority", LinguisticVariable([P0,P1, P2, P3, P4], universe_of_discourse=[0, 1]))

    FS.add_rules([

        "IF (M1 IS Very_Short) AND (M2 IS Very_Long) THEN (priority IS very_high)",

        "IF (M1 IS Very_Short) AND (M2 IS Long) THEN (priority IS very_high)",

        "IF (M1 IS Very_Short) AND (M2 IS Medium) THEN (priority IS high)",

        "IF (M1 IS Very_Short) AND (M2 IS Very_Short) THEN (priority IS medium)",

        "IF (M1 IS Very_Short) AND (M2 IS Short) THEN (priority IS medium)",

        "IF (M1 IS Short) AND (M2 IS Very_Long) THEN (priority IS very_high)",

        "IF (M1 IS Short) AND (M2 IS Long) THEN (priority IS high)",

        "IF (M1 IS Short) AND (M2 IS Medium) THEN (priority IS high)",

        "IF (M1 IS Short) AND (M2 IS Very_Short) THEN (priority IS low)",

        "IF (M1 IS Short) AND (M2 IS Short) THEN (priority IS medium)",

        "IF (M1 IS Medium) AND (M2 IS Medium) THEN (priority IS medium)",

        "IF (M1 IS Medium) AND (M2 IS Very_Short) THEN (priority IS low)",

        "IF (M1 IS Medium) AND (M2 IS Short) THEN (priority IS low)",

        "IF (M1 IS Medium) AND (M2 IS Long) THEN (priority IS high)",

        "IF (M1 IS Medium) AND (M2 IS Very_Long) THEN (priority IS high)",

        "IF (M1 IS Long) AND (M2 IS Long) THEN (priority IS medium)",

        "IF (M1 IS Long) AND (M2 IS Very_Long) THEN (priority IS medium)",

        "IF (M1 IS Long) AND (M2 IS Medium) THEN (priority IS low)",

        "IF (M1 IS Long) AND (M2 IS Short) THEN (priority IS very_low)",

        "IF (M1 IS Long) AND (M2 IS Very_Short) THEN (priority IS very_low)",

        "IF (M1 IS Very_Long) AND (M2 IS Very_Short) THEN (priority IS very_low)",

        "IF (M1 IS Very_Long) AND (M2 IS Short) THEN (priority IS very_low)",

        "IF (M1 IS Very_Long) AND (M2 IS Medium ) THEN (priority IS low)",

        "IF (M1 IS Very_Long) AND (M2 IS Long) THEN (priority IS low)",

        "IF (M1 IS Very_Long) AND (M2 IS Very_Long) THEN (priority IS medium)",


    ])

    return FS

def Horn(J, fs):
    order = []
    k=0



    global t_second_machine
    t1 = min(task[0] for task in J)
    A = set()
    while J or A:
        k = k + 1
        A |= {task for task in J if task[0] <= t1}
        J -= A


        if not J:
            t2 = float('inf')
        else:
            t2 = min(task[0] for task in J)


        if A:
            print("A",A)
            priority = 0
            for a in A:
                    fs.set_variable("M1", a[1])
                    fs.set_variable("M2", a[2])
                    current_priority = fs.inference()["priority"]
                    if current_priority >= priority:
                        priority = current_priority
                        task = a
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
def run_algorithm(input_data):
    global schedule, schedule2, A, t_second_machine
    A = set()
    schedule = {}
    schedule2 = {}
    t_second_machine = 0
    data = []
    k=1
    p1 = 0
    p2 = 0
    for numbers in input_data:
        element = tuple(numbers) + (k,)
        data.append(element)
        schedule[k] = []
        schedule2[k] = []
        k=k+1
        p1= p1+ element[1]
        p2 = p2 + element[2]
    fs = create_FS(p1, p2, p1 // len(data), p2 // len(data))
    Horn(set(data), fs)
    res = {}
    res['result_1'] = schedule
    res['result_2'] = schedule2
    res['c_max'] = t_second_machine
    return res
if __name__ == '__main__':
    all_tasks=[]
    result = defaultdict(list)
    result2 = defaultdict(list)
    print("Nazwa pliku:")
    file_name = input()
    k=1
    data=[]
    p1=0
    p2 = 0
    with open(file_name, 'r') as file:
        for numbers in file:
            element = tuple(map(int, numbers.split())) + (k,)
            all_tasks.append(element)
            schedule[k] = []
            schedule2[k] = []
            k=k+1
            p1= p1+ element[1]
            p2 = p2 + element[2]
    print(p2)
    fs = create_FS(p1,p2, p1 // len(all_tasks), p2 // len(all_tasks))
    Horn(set(all_tasks), fs)
    print("Rozwiązanie:")
    print(schedule)
    print(schedule2)