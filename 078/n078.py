for i in range(50000,60000):
    print(i)
    if (npartitions(i) % 1000000) == 0:
        print("Answer! ", npartitions(i))
        break
