from scapy.all import *
import sys
from docopt import docopt

__doc__ = """Traceroute implementation in Python

Usage:
    traceroute.py -d <destination>
    traceroute.py --version

Options:
    --version: Show version
"""

def trace_route(destination):
    flag = True
    ttl = 1
    hops = []
    while flag:
        ans, unans = sr(IP(dst=destination, ttl=ttl)/ICMP(), verbose=0)
        if ans.res[0][1].type == 0:
            flag = False
        else:
            hops.append(ans.res[0][1].src)
            ttl += 1

    i = 1
    print "IPs traced in the path:"
    for hop in hops:
        print "[{}] {}".format(i, hop)
        i += 1

if __name__=="__main__":
    arguments = docopt(__doc__, version = "3.2.2")

    destination = arguments["<destination>"]

    try:
        trace_route(destination)
    except Exception as e:
        print "An error ocurred"
