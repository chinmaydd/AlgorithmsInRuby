from scapy.all import *
import sys
from docopt import docopt

__doc__ = """Report all MAC and IP (alive) addresses in the subnet

Usage:
    mac_checker.py [-d] [<destination>]
    mac_checker.py --version

Options:
    --version: Show version
"""

def start_subnet_trace():
    print "HALLELUJAH"

def start_trace(destination):
    ans, unans = srp(Ether(dst="ff:ff:ff:ff:ff:ff")/ARP(pdst=destination),timeout = 2, verbose=0)

    for snd, rcv in ans:
        print rcv.sprintf("{0} - {1}".format(Ether.src, ARP.src))

if __name__=="__main__":
    arguments = docopt(__doc__, version = "3.2.2")

    destination = arguments["<destination>"]

    if destination:
        start_trace(destination)
    else:
        start_subnet_trace()
