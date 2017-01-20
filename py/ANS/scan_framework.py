from scapy.all import *
import sys
import pdb
from docopt import docopt

sys.stdout = open('results.txt', 'w')

__doc__ = """Poppy: Port scanning framework.

Usage:
    ack_scan.py (-s|--scan) <scan> (-p|--port) <port> (-d|--destination) <destination>
    ack_scan.py --version

Options:
    --version: Show version.
    --scan: Scanning technique to be used.
            1. ack
            2. ping
            3. xmas
    --port: ',' seperated ports or range of ports specified by '-'
    --destination: Destination to scan
"""

def ack(destination, ports):
    for dst_port in ports:
        ans, unans = sr(IP(dst = destination)/TCP(flags="A",dport=dst_port), timeout=10, verbose=0)

        for s, r in ans:
            if s[TCP].dport == r[TCP].sport:
                print str(s[TCP].dport) + " is unfiltered."

        for s in unans:
            print str(s[TCP].dport) + " is filtered."

def xmas(destination, ports):
    for dst_port in ports:
        resp = sr1(IP(dst=destination)/TCP(dport=dst_port,flags="FPU"),timeout=10, verbose=0)
    
        if (str(type(resp))=="<type 'NoneType'>"):
            print "Port " + str(dst_port) + " is open|filtered."
        elif(resp.haslayer(TCP)):
            if(resp.getlayer(TCP).flags == 0x14):
                print "Port " + str(dst_port) + " is closed."

def ping(destination, ports):
    for dst_port in ports:
        ans, unans = sr(IP(dst=destination)/TCP(flags="S", dport=dst_port), timeout=10, verbose=0)
    
        for s, r in ans:
            print str(dst_port) + " is alive."

        if not ans:
            print str(dst_port) + " did not respond back."

def get_destination_ports(port_str):
    if '-' in port_str:
        a = port_str.split('-')
        return list(range(int(a[0]), int(a[1])))
    else:
        return [int(x) for x in port_str.split(',')]

if __name__=="__main__":
    arguments = docopt(__doc__, version = "3.2.2")

    ports = get_destination_ports(arguments['<port>'])
    destination = arguments['<destination>']
    technique = arguments['<scan>']

    try:
        locals()[technique](destination, ports)
    except Exception as e:
        print "An error ocurred."
