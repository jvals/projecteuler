'''
You are given the following information, but you may prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

There is one monday in a week
There are seven days in a week
There are N days in a month
There are 12 months in a year

'''
import time
yearDict = {1:31, 2:28, 3:31, 4:30, 5:31, 6:30, 7:31, 8:31, 9:30, 10:31, 11:30, 12:31}
leapYearDict =  {1:31, 2:29, 3:31, 4:30, 5:31, 6:30, 7:31, 8:31, 9:30, 10:31, 11:30, 12:31}

def isLeapYear(n):
	if not (n%4 and n%400):
		return True
	return False

start = time.time()
answer = 0
dayOfWeek = 1

for year in range(1900,2001):
	dic = yearDict if not isLeapYear(year) else leapYearDict
	for month in range(1,13):
		for dayOfMonth in range(1,dic.get(month)+1):
			#print(dayOfMonth,'\t',dayOfWeek,'\t',year)
			if(dayOfMonth == 1 and dayOfWeek == 7 and year != 1900):
				answer += 1
			if not dayOfWeek%7:
				dayOfWeek = 0
			dayOfWeek += 1
	month = 1

end = time.time()

print(answer," time: ",end-start)


