with open("input.txt", "r") as f:
    matrix = [list(i.strip()) for i in f.readlines()]
    print([set(k) for k in matrix])

dimx = len(matrix[0])
dimy = len(matrix)

def xmassearch(matrix, i, j):
    UL = matrix[i-1][j-1]
    UR = matrix[i-1][j+1]
    DL = matrix[i+1][j-1]
    DR = matrix[i+1][j+1]
    
    return set([UL,DR])==set("MS") and set([UR,DL])==set("MS")


count = 0
print(dimx, dimy)
for i in range(1,dimx-1):
    for j in range(1,dimy-1):
        if matrix[i][j]=="A": count += xmassearch(matrix, i, j)

print(count)