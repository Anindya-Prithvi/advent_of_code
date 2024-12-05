with open("input.txt", "r") as f:
    matrix = [list(i) for i in f.readlines()]
    print([len(k) for k in matrix])

def wordsearch(matrix):
    def backsearch(matrix, i, j):
        if j-3 < 0 or j + 1 > len(matrix[0]): return False
        return "".join((matrix[i][j-3:j+1])[::-1])=="XMAS"
    def frontsearch(matrix, i, j):
        if j+4 > len(matrix[0]): return False
        return "".join(matrix[i][j:j+4])=="XMAS"
    def downsearch(matrix, i, j):
        if i+3>=len(matrix): return False
        return "".join(''.join(matrix[i+k][j] for k in range(4)))=="XMAS"
    def upsearch(matrix, i, j):
        if i-3<0 or i>=len(matrix): return False
        return "".join(''.join(matrix[i-3+k][j] for k in range(4)))[::-1]=="XMAS"
    def diagdownrightsearch(matrix, i, j):
        if i+3>=len(matrix) or j+3>=len(matrix[0]): return False
        return "".join([matrix[i+k][j+k] for k in range(4)])=="XMAS"
    def diagdownleftsearch(matrix, i, j):
        if i+3>=len(matrix) or j-3<0: return False
        return "".join([matrix[i+k][j-k] for k in range(4)])=="XMAS"
    def diaguprightsearch(matrix, i, j):
        if i-3<0 or j+3>=len(matrix[0]): return False
        return "".join([matrix[i-k][j+k] for k in range(4)])=="XMAS"
    def diagupleftsearch(matrix, i, j):
        if i-3<0 or j-3<0: return False
        return "".join([matrix[i-k][j-k] for k in range(4)])=="XMAS"
    
    count = 0
    for a in range(len(matrix)):
        for b in range(len(matrix[0])):
            count += backsearch(matrix, a,b)+frontsearch(matrix, a,b)+upsearch(matrix,a,b)+downsearch(matrix,a,b)
            count += diagdownleftsearch(matrix,a,b)+diagdownrightsearch(matrix,a,b)+diagupleftsearch(matrix,a,b)+diaguprightsearch(matrix,a,b)
    print(count)

wordsearch(matrix)