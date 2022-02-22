"""
    1 : gillar {cheese peppers}
        ogillar {}
    2 : gillar {basil}
        ogillar {pineapple}
    3 : gillar {mushrooms tomatoes}
        ogillar {basil}
"""


# reads n lines from std in
ingredients = set()
customer_like = {}
customer_dislike = {}
disliked = set()
liked = set()

'''
a [1], []
b [2], [1]
c [2], []
d [3], [2]
e [1], []
f [2], [1]

1 -> +1 kund
2 -> +2 kund
'''

def test_ingredients():
  discarded = set()
  # assume garbage data
  ingredient_score = {"garbage":"data"}
  while ingredient_score:
    print(f'{len(ingredient_score)=}')
    print(f'{len(customer_like)=}')
    print(f'{len(ingredients)=}')
    print(f'{len(discarded)=}')
    print(f'{discarded=}')
    print('-'*50)
    # track best ingredient

    ingredient_score = {}
    for ingredient in ingredients:
      liked = 0
      disliked = 0
      for key in customer_like.keys(): 
        if key in discarded:
            continue
        if ingredient in customer_dislike[key] and customer_dislike[key].isdisjoint(result):
          disliked += 1
        elif not (customer_like[key] <= result) and set([ingredient]) == (customer_like[key] - result):
          liked += 1
      if(liked >= disliked):
        ingredient_score[ingredient] = liked - disliked
    if ingredient_score:
      winner = max(ingredient_score, key=ingredient_score.get)
      print(f'{ingredient_score=}')
      print(f'{winner= }')
      #print(f"{winner} seems nice, it gave {ingredient_score[winner]} new customers")
      result.add(winner)
      for key in customer_dislike:
          if winner in customer_dislike[key]:
              discarded.add(key)
              print(f'{key} disliked {winner} with a score of {ingredient_score[winner]=}')
    
    ingredients.difference_update(result)


'''
a [1], [2]
b [2, 3], [1]
c [2, 3], [1]
'''


def satisfied(customer_like,customer_dislike,result):
  correct = 0
  for customer in customer_dislike:
    if customer_like[customer] <= result and customer_dislike[customer].isdisjoint(result):
      correct+=1
  return correct
files = ["test.in.txt","a_an_example.in.txt","b_basic.in.txt","c_coarse.in.txt","d_difficult.in.txt","e_elaborate.in.txt"]
for file_name in files:
  print('='*50)
  print(file_name)
  print('='*50)
  
  ingredients = set()
  customer_like = {}
  customer_dislike = {}
  disliked = set()
  liked = set()
  with open(file_name) as f:
    n = int(f.readline())
    i = 0
    for line in f:

      inp = line.split()[1:len(line)]
      if i % 2 == 0:
        dct = customer_like
        liked.update(inp)
      else:
        dct = customer_dislike
        disliked.update(inp)
      dct[i - i % 2] = set(inp)
      ingredients.update(inp)
      i+=1
  print(f'{len(ingredients)=}')
  result = liked - disliked
  ingredients -= (disliked - liked)
  ingredients -= result
  naive_satisfied = satisfied(customer_like,customer_dislike,result)
  print(naive_satisfied)
  print(f"{len(result)=}")
  test_ingredients()
  print(f"{len(result)=}")
  less_naive_satisfied = satisfied(customer_like,customer_dislike,result)
  print(less_naive_satisfied)

  #print(len(ingredients))
  #print(result,ingredients,f"improvement = {less_naive_satisfied-naive_satisfied}")

  with  open(f'output_{file_name[:-7]}.txt', 'w') as f:
    f.write(str(len(ingredients)))
    for i in ingredients:
      f.write(f' {i}')
