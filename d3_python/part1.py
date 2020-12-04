import sys
import os
import sys
import fileinput
import shutil, errno

f = open('input','r')

def count_trees(slope,right,down):
    x = 0
    tree_count = 0

    for slope_slice in range(len(slope)):
        if down != 1 and slope_slice %2 != 0:
            continue
        len_slice = len(slope[slope_slice])-1
        slope_expand = slope[slope_slice].strip() *1000
        for slope_square in range(len(slope_expand)):
            if slope_square == x:
                if slope_expand[x] == "#":
                    tree_count +=1
        x+=right
    
    print tree_count
    return tree_count
    
    

slope = f.readlines()
print count_trees(slope,1,1) * count_trees(slope,3,1) * \
    count_trees(slope,5,1) * count_trees(slope,7,1) * count_trees(slope,1,2)
