def hello():
    print('Hello,', end='')
    yield  # 여기에서 중단, 재개 ❶
    print('World!')
    yield  # 여기까지 실행 ❷

h = hello()  # 이터레이터를 생성
h.__next__() # 1까지 실행하고 중단
h.__next__() # 1에서 재개하고 2까지 실행
