import sys

# mock = ["135", "413", "471", "#457", "713", "#435", "735", "473", "#415", "715"]



def main():
    participants = set()
    relations = []
    for line in sys.stdin:
    # for line in mock:
        if line[0] != '#':
            participants.update(list(line.strip()))
            relations.append(list(line.strip()))

    ndescendants = len(participants)
    print("NDESCENDANTS:", ndescendants)
    print("PARTICIPANTS:", participants)
    print("RELEATIONS:", relations)

    descendants = dict.fromkeys(list(participants))
    print("DESCENDANTS:", descendants)

    initializeDescendants(descendants, relations)
    print("DESCENDANTS:", descendants)

    for key in descendants:
        if descendants[key]:
            extendDescendantsOfKey(descendants, key)

    print("DESCENDANTS:", descendants)
    for key in descendants:
        print(key, ": ", end="", sep="")
        for value in descendants[key]:
            print(value, ' ', end="", sep="")
        print()

def initializeDescendants(descendants, relations):
    for key in descendants:
        descendants[key] = set()
    
    for i in range(len(relations)):
        descendants[relations[i][2]].add(relations[i][1])
        descendants[relations[i][2]].add(relations[i][0])

        descendants[relations[i][1]].add(relations[i][0])

    
def extendDescendantsOfKey(descendants, key):
    newMembers = set()
    for value in descendants[key]:
        if descendants[value]:
            extendDescendantsOfKey(descendants, value)
            newMembers.update(descendants[value])
    descendants[key].update(newMembers)
            

    


if __name__ == '__main__':
    main()
