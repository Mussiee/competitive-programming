from collections import defaultdict
class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        
        r = defaultdict(set)
        c = defaultdict(set) 

        squares = defaultdict(set) 

        for i in range(0,9):
            for j in range(0,9):
                temp = board[i][j] 
                if temp == '.': 
                    continue 
                
                if( temp in r[i] ) or (temp in c[j]) or (temp in squares[(i // 3 , j//3)]): 
                    return False

                r[i].add(temp)
                c[j].add(temp) 
                squares[(i//3, j//3)].add(temp)
        
        return True