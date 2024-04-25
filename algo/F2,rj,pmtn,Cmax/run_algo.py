import importlib
import importlib.util
import sys
import json
import time

# example usage on Windows:
# python run_algo.py Johnson.py input.txt

if len(sys.argv) < 3:
    print("Usage: python3 " + sys.argv[0] + " <algo_name> <input_file> <optional output_file>")
    sys.exit(1)

sys.dont_write_bytecode = True

module_name = "__api__"
file_name = sys.argv[2]

print("Reading input file...")

input_data = []
with open(file_name, 'r') as file:
    for numbers in file:
        input_data.append(list(map(int, numbers.split())))

print("Parsing algorithm file...")

spec = importlib.util.spec_from_file_location(module_name, sys.argv[1])
module = importlib.util.module_from_spec(spec)
sys.modules[module_name] = module
spec.loader.exec_module(module)

timeStart = time.time()
result = module.run_algorithm(input_data)
timeEnd = time.time()

print("Execution time: ", timeEnd - timeStart)
result = json.loads(json.dumps(result))

if (len(sys.argv) == 4):
    with open(sys.argv[3], 'w') as file:
        file.write("Zadania na pierwszej maszynie: \n")
        for key, value in result["result_1"].items():
            file.write(f'{key}: {value}\n')
        file.write("Zadania na drugiej maszynie: \n")
        for key, value in result["result_2"].items():
            file.write(f'{key}: {value}\n')
    print("Results saved to ", sys.argv[3])
else:
    print("Zadania na pierwszej maszynie: ")
    for key, value in result["result_1"].items():
        print(f'{key}: {value}')
    print("Zadania na drugiej maszynie: ")
    for key, value in result["result_2"].items():
        print(f'{key}: {value}')
