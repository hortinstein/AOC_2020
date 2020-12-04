import sys
import os
import sys
import fileinput
import shutil, errno
import string 

f = open('input','r')
record_raw = f.readlines()

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def len_min_max(field,len_f,min,max):
    if len(field) != len_f: return False
    int_field = int(field.split(':')[1])
    if int_field>max: return False
    if int_field<min: return False
    return True


'''
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.
'''
def check_field_data(record_line):
    split = record_line.split(' ')
    print (split)
    for field in split:
        print (field)
        if ('byr' in field):
            if not len_min_max(field,8,1920,2002): 
                print ('byr failure')
                return False
        if ('iyr' in field):
            if not len_min_max(field,8,2010,2020): 
                print ('iyr failure')
                return False
        if ('eyr' in field):
            if not len_min_max(field,8,2020,2030):  
                print ('eyr failure')
                return False
        if ('hgt' in field):
            if 'cm' in field:
                if int(field[4:6])<150 and int(field[4:6])>193:  
                    print ('hgt failure')
                    return False 
            elif 'in' in field:
                if int(field[4:5])<59 and int(field[4:5])>76:  
                    print ('hgt failure')
                    return False
            else: return False 
        if ('hcl' in field):
            if field[4] != '#': 
                print ('hcl failure')
                return False
            if len(field) != 11 and not all(c in string.hexdigits for c in field[5:10]): 
                print ('hcl failure')
                return False
        if ('ecl' in field):
            if not len(field) != 7 and field[4:7] not in ["amb","blu","brn","gry","grn","hzl","oth"]:  
                print ('ecl failure')
                return False
        if ('pid' in field):
            if not len(field) != 13 and not field[4:].isdigit(): 
                print ('pid failure')
                return False
    return True

"""
byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
"""
def check_fields(record_line):
    
    field_list = ['byr','iyr','eyr','hgt','hcl','ecl', 'pid']
    
    for field in field_list:
        if field not in record_line:
            print (bcolors.FAIL,field,bcolors.ENDC)
            return False

    check_data =  check_field_data(record_line)
    if not check_data: print (bcolors.FAIL,"FAIL",bcolors.ENDC)

    return check_data

def consolidate_records(raw):
    records = [""]
    i = 0
    for line in record_raw:
        if line == '\n':
            records.append("")
            i+=1
        else: 
            records[i] += line.rstrip() + " "
    return records

valid_records = 0
records =consolidate_records(record_raw)
for record in records:
    #print (record)
    if (check_fields(record)):
        valid_records+=1
print (valid_records)