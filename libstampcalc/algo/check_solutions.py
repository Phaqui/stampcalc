import sys
import argparse

dot = lambda a, b: sum(x * y for x, y in zip(a, b))
check_solution = lambda price, stamps, factors: price == dot(factors, stamps)

def parse_args():
    parser = argparse.ArgumentParser(description="Sanitycheck stampcalc solutions")
    parser.add_argument("-s", "--silent", action="store_true",
            help="don't print anything (errno is still set appropriately)")
    parser.add_argument("file", type=argparse.FileType("r"),
            help="input file to read from (use - to read from stdin)")
    return parser.parse_args()

def parse_input(input_lines):
    it = iter(input_lines)

    price, *stamps = [int(x) for x in next(it).strip().split(" ")]
    solutions = [tuple(int(x) for x in line.strip().split(" ")) for line in it]

    return price, stamps, solutions

def main():
    args = parse_args()
    price, stamps, proposed_solutions = parse_input(args.file.readlines())
    good, bad, wrong_len, n_duplicates = 0, 0, 0, 0

    checked_solutions = set()
    for possible_solution in proposed_solutions:
        if possible_solution in checked_solutions:
            n_duplicates += 1
            continue
        checked_solutions.add(possible_solution)
        if len(possible_solution) != len(stamps):
            wrong_len += 1
            continue
        if check_solution(price, stamps, possible_solution):
            good += 1
        else:
            bad += 1
    
    total = good + bad + wrong_len + n_duplicates
    all_unique = len(checked_solutions) == good
    all_good = total == good
    if not args.silent:
        if all_good and all_unique:
            print(f"Given only unique solutions, and they all checked out ({total=})")
        else:
            print(f"{good} solutions checked out, {bad} solutions were incorrect, {wrong_len} given solutions had incorrect lenght, while {n_duplicates} were duplicates. ({total=})")
    return 0 if all_good else 1

if __name__ == "__main__":
    raise SystemExit(main())
