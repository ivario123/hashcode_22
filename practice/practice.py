import random

customers = []
ingredients = set()
like_count = {}
disl_count = {}
liked = set()
disliked = set()


def upd(arr, key):
    if key in arr:
        arr[key] += 1
    else:
        arr[key] = 1


def get_satisfied(ingredients):
    likes = 0
    for customer in customers:
        if customer[0].issubset(ingredients) and customer[1].isdisjoint(ingredients):
            likes += 1
    return likes


def type_1():
    ingredients = liked
    return ingredients


def type_2(mult=1):
    ingredients = set()
    prev_satisfied = 0
    customers.sort(key=lambda cust: (len(cust[0]) * mult) + len(cust[1]))
    for customer in customers:
        clone = ingredients.copy()
        clone.update(customer[0])
        satisfied = get_satisfied(clone)
        if satisfied > prev_satisfied:
            prev_satisfied = satisfied
            ingredients = clone
    return ingredients


if __name__ == "__main__":
    clients = int(input())

    for _ in range(0, clients):
        cust_dislikes = frozenset()
        cust_likes = frozenset()
        like_in = input().split()
        if int(like_in[0]) > 0:
            ingredients.update(like_in[1:])
            cust_likes = frozenset(like_in[1:])
            liked.update(like_in[1:])
            upd(like_count, cust_likes)

        dislike_in = input().split()
        if int(dislike_in[0]) > 0:
            ingredients.update(dislike_in[1:])
            cust_dislikes = frozenset(dislike_in[1:])
            disliked.update(dislike_in[1:])
            upd(disl_count, cust_dislikes)

        customers.append((cust_likes, cust_dislikes, set()))

    for customer in customers:
        cust_ingredients = frozenset(customer[0].union(customer[1]))
        customer[2].update(ingredients.difference(cust_ingredients))

    ingredients = type_2(1)
    print(str(len(ingredients)) + " " + " ".join(ingredients))
