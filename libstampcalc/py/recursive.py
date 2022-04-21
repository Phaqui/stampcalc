def find_all_solutions(price, stamps):
    stamps = list(set(stamps))
    stamps.sort()
    stamps.reverse()

    current_factors = [0] * len(stamps)

    nchecks = 0

    def _solutions(price, idx, current_factors, stamps):
        nonlocal nchecks
        if len(stamps) == 1:
            nchecks += 1
            last = price / stamps[0]
            if last.is_integer() and last >= 0:
                yield current_factors[:-1] + [int(last)]
        else:
            biggest, rest = stamps[0], stamps[1:]
            max_num_biggest = price // biggest
            for i in range(max_num_biggest + 1):
                current_factors_local = current_factors.copy()
                current_factors_local[idx] = i
                yield from _solutions(
                        price - stamps[0] * i, idx + 1, current_factors_local, rest)

    yield from _solutions(price, 0, [0] * len(stamps), stamps)
    return nchecks

if __name__ == "__main__":
    import sys
    import argparse
    parser = argparse.ArgumentParser(description="stamp calculator")
    parser.add_argument("price", type=int)
    parser.add_argument("stamps", type=int, nargs="+")
    parser.add_argument("-s", "--no-summary", help="don't show summary", action="store_true")
    parser.add_argument("-r", "--no-results", help="don't show results", action="store_true")
    args = parser.parse_args()

    stamps = [int(stamp) for stamp in args.stamps]
    stamps = list(set(args.stamps))
    stamps.sort()
    stamps.reverse()
    stamps_s = " ".join(str(x) for x in stamps)
    price = args.price

    print(f"{price} {stamps_s}")

    n_checks, n_solutions = 0, 0

    def set_nchecks(val):
        global n_checks
        n_checks = val

    def handle_return(generator, func):
        retval = yield from generator
        func(retval)

    solutions = find_all_solutions(price, stamps)
    for solution in handle_return(solutions, set_nchecks):
        n_solutions += 1
        if not args.no_results:
            print(" ".join(str(x) for x in solution))

    if not args.no_summary:
        print(f"{n_solutions} solutions found (in {n_checks} checks)")

