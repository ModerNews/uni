#!/usr/bin/env python3
import urllib.request as request 
import getopt
import sys


def get(url):
    with request.urlopen(url) as response:
        data = response.read()
        try:
            return data.decode('utf-8')
        except UnicodeDecodeError:
            return data.decode('cp1250')
        except:
            raise Exception("Unknown encoding")


optlist, arg = getopt.getopt(sys.argv[1:], "", [])
url = arg[0]
print(get(url)[:500])
