#
# Some attempt at an interative approach, that would behave in the same
# way as the recursive one in recursive.py
from array import array

dot = lambda a, b: sum(x * y for x, y in zip(a, b))
calc_max_factors = lambda price, stamps: array('L', [int(price / x) for x in stamps])

def try_solve(price, stamps, given_factors):
    """Given

        `Factors` = [F_n, F_n-1, F_n-2, ..., F_0]
        `Stamps` = [S_n, S_n-1, S_n-2, ..., S_0]

       Tries to solve the equation
        
        price = F_n * S_n + F_n-1 * S_n-1 + F_n-2 * S_n-2 + ... + F_0 * S_0

        price - (F_n * S_n + F_n-1 + S_n-1 + F_n-2 * S_n-2 + ... = F_0 * S_0

        F_0 = (price - (F_n * S_n + F_n-1 + S_n-1 + ... + F_1 + S_1)) / S_0
    """
    solved = (price - dot(given_factors[:-1], stamps[:-1])) / stamps[-1]
    s = f"{given_factors=} ; {stamps=}"
    assert solved >= 0, f"solved can't be negative (was {solved}) [{s}]"
    if solved.is_integer():
        return int(solved)

class Ref:
    def __init__(self, arg):
        self.mv = memoryview(arg)

    @classmethod
    def of_length(cls, n: int):
        arr = array('L', [0] * n)
        self = cls(arr)
        return self

    def __str__(self):
        return "Ref<[" + ', '.join(str(x) for x in self.tolist()) + "]>"
    def __repr__(self):
        return self.__str__()

    def __getitem__(self, key):
        if isinstance(key, slice):
            return Ref(self.mv[key])
        else:
            return self.mv[key]

    def __len__(self):
        return len(self.mv)

    def __add__(self, other):
        if isinstance(other, list):
            newlist = self.mv.tolist() + other
            newarr = array('L', newlist)
            new = Ref(newarr)
            return new
        else:
            type_s = str(type(other))[8:-2]
            err = f"unsupported operand type(s) for +: 'Ref' and '{type_s}'"
            raise TypeError(err)


    def __setitem__(self, key, value):
        self.mv[key] = value
    def tolist(self):
        return self.mv.tolist()

def trysolve(price, stamp):
    sol = price / stamp
    return int(sol) if sol.is_integer() and sol > 0 else None

def increment_current(current, max_factors):
    for i in reversed(range(0, len(max_factors))):
        if current[i] < max_factors[i]:
            current[i] += 1
            return i;
        else:
            current[i] = 0
    return -1

def skip_last(current, max_factors):
    assert isinstance(max_factors, array), "max_factors must be array"
    max_factors = Ref(max_factors)
    current[-1] = 0
    return increment_current(current[:-1], max_factors[:-1])

def solutions(price, stamps):
    def print_solution(sol_vec):
        if isinstance(sol_vec, memoryview) or isinstance(sol_vec, array):
            sol_vec = sol_vec.tolist()
        terms = ' + '.join(f"{a} * {b}" for a,b in zip(sol_vec, stamps))
        print(f"solution: {price} = {terms}")

    L = len(stamps)
    current = Ref.of_length(len(stamps))
    stamps = stamps.copy()
    stamps.sort()
    stamps.reverse()
    stamps = array('L', stamps)

    answers = set()

    pth = price
    max_factors = calc_max_factors(pth, stamps)
    print(f"{max_factors=}")
    nchecks, nsolutions = 0, 0
    imd = increment_current(current, max_factors)
    while imd != -1:
        if imd < L - 1:
            pth = price - dot(current[:imd+1], stamps[:imd+1])
            if pth <= 0:
                # we already hit a case where `current` is too high
                # to produce any solutions, so we just go next
                pass
            else:
                local_max_factors = calc_max_factors(pth, stamps[:imd+1])
                #print(f"{pth=} {imd=} {stamps[:imd+1]=}, {local_max_factors=}")
                local_current = Ref.of_length(len(local_max_factors))

                digit = 1
                while digit != -1:
                    nchecks += 1
                    if pth == dot(local_current, stamps[:imd+1]):
                        sol = tuple( (local_current + [0]).tolist() )
                        answers.add(sol)
                        nsolutions += 1
                    digit = increment_current(local_current, local_max_factors)
                
            imd = increment_current(current, max_factors)
        elif imd == L - 1:
            nchecks += 1
            solved = trysolve(pth, stamps[-1])
            if solved is not None:
                sol = tuple( (current[:-1] + [solved]).tolist() )
                answers.add(sol)
                nsolutions += 1
            imd = skip_last(current, max_factors)

    nans = len(answers)
    writeout(price, stamps, answers)
    print(f"found {nans} unique solutions ({nsolutions} total) in {nchecks} checks")

def solutions2(price, current_factors, stamps):
    if len(stamps) == 1:
        sol = price / stamps[0]
        if sol.is_integer() and sol >= 0:
            yield current_factors[:-1] + [int(sol)]
    else:
        biggest, rest = stamps[0], stamps[1:]
        max_num_biggest = price // biggest
        for i in range(max_num_biggest + 1):
            current_factors = [i] + current_factors[1:]
            yield from solutions2(price - stamps[0] * i, current_factors, rest)

def writeout(price, stamps, solutions):
    with open("OUTPUT", "w") as f:
        f.write(str(price) + " ")
        f.write(" ".join(str(s) for s in stamps) + "\n")
        for sol in solutions:
            f.write(" ".join(str(x) for x in sol) + "\n")


if __name__ == "__main__":
    import sys
    price, stamps = 30, [5, 10, 20]
    if len(sys.argv) > 2:
        price, stamps = int(sys.argv[1]), [int(st) for st in sys.argv[2:]]

    #solutions(price, stamps)

    stamps.reverse()
    print(f"{price=} given stamps: {stamps}")
    answers = list(solutions2(price, [0] * len(stamps), stamps))
    #for answer in answers:
    #    print(answer)
    print(f"{len(answers)} solutions found")
