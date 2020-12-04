import sys
import os
import sys
import fileinput
import shutil, errno

f = open('input','r')

slope = f.readlines()

x = 0
y = 0
tree_count = 0

for slope_slice in slope:
    len_slice = len(slope_slice)-1
    for slope_square in range(len(slope_slice)):
        if slope_square == x % len_slice:
            if slope_slice[x % len_slice] == "#":
                tree_count +=1
                print 'X',
            else:
                print '0',
        else:
            print slope_slice[slope_square],
    print x % len_slice
    x+=3
    
print tree_count
    
    