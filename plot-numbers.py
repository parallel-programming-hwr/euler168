import matplotlib.pyplot as plt


def all_the_same(digits) -> bool:
    first = digits[0]
    for dig in digits:
        if dig != first:
            return False
    return True


with open('./rotatable.txt', 'r') as file:
    numbers = [int(num.strip('\n')) for num in file.readlines()]
    numbers.sort()
    x = []
    y = []
    rem_three = {0: 0, 1: 0, 2: 0}
    for i in numbers:
        i_dig = list(str(i))
        if not all_the_same(i_dig):
            print(i % 3)
            last = i_dig.pop()
            i_dig.insert(0, last)
            #x.append(i%3)
            rem_three[i%3] += 1
            x.append(int(''.join(i_dig)))
            y.append(i)
    fix, axs = plt.subplots(1, 2)
    print(x, y)
    axs[0].scatter(x, y)
    axs[0].set(xlabel='right rotation', ylabel='number')
    axs[0].set_title('Numbers and their right rotation')
    axs[0].grid()
    axs[1].bar([0, 1, 2], [rem_three[0], rem_three[1], rem_three[2]])
    axs[1].set(xlabel='num % 3', ylabel='count')
    axs[1].set_title('Remainder of the number and 3')
    plt.show()