from math import prod

with open("../6.txt") as f:
    lines = [line.rstrip("\n") for line in f]

nums = [[int(x) for x in line.split()] for line in lines[:-1]]
ops = lines[-1].split()

print(
    sum(
        sum(column) if ops[i] == "+" else prod(column)
        for i, column in enumerate(zip(*nums))
    )
)

nums = []
problem: list[int] = []
for column in zip(*lines[:-1]):
    col = "".join(column).strip()
    if not col:
        nums.append(problem)
        problem = []
        continue
    problem.append(int(col))
if problem:
    nums.append(problem)

print(
    sum(sum(column) if ops[i] == "+" else prod(column) for i, column in enumerate(nums))
)
