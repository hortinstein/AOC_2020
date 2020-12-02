import sys
import os
import sys
import fileinput
import shutil, errno

f = open('input','r')

f_lines = f.readlines()

#remove the new lines and make a list of the dates
dates_str = list(map(str.strip,f_lines))

dates = list(map(int, dates_str)) 

dates.sort()

for date in dates:
    if date > 1000:
        break
    for date2 in dates:
        if date+date2 == 2020:
            print ("P1 date {}, date2 {}, multple:{}".format(date,date2,date*date2)) 

        for date3 in dates:
            if date+date2+date3 == 2020:
                print ("P2 date {}, date2 {}, date3 {}, multple:{}".format(date,date2,date3,date*date2*date3)) 
