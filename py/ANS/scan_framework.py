import sys
from docopt import docopt
import logging

# Suppress all warnings
logging.getLogger("scapy.runtime").setLevel(logging.ERROR)
from scapy.all import *

# sys.stdout = open('results.txt', 'w')
__doc__ = """Poppy: Port scanning framework.

Usage:
    ack_scan.py <destination> --scan <scan> [--port <port>] [--verbose] [--packet-trace]
    ack_scan.py --version

Option:
    --scan: Scan options:
            1. tcp_conn: TCP Connect scan
            2. ack: ACK Scan
            3. fin: FIN Scan
            4. xmas: XMAS Scan
"""

# TCP Connect
# The client sends the first handshake using the SYN flag and port, 
# to connect to the server in a TCP packet.
# If the server responds with a RST instead of a SYN-ACK then the particular port is closed.
def tcp_conn(destination, ports, verbose):        
    print "PORT\tSTATE"
    for dst_port in ports:
        src_port = RandShort()
        resp = sr1(IP(dst=destination)/TCP(sport=src_port,dport=dst_port,flags="S"),timeout=2, verbose=verbose)
        
        if(str(type(resp))=="<type 'NoneType'>"):
            pass
        elif(resp.haslayer(TCP)):
            if(resp.getlayer(TCP).flags == 0x12):
                send_rst = sr(IP(dst=destination)/TCP(sport=src_port,dport=dst_port,flags="AR"),timeout=10, verbose=verbose)
                print str(dst_port) + "/tcp\topen"
            elif (resp.getlayer(TCP).flags == 0x14):
                pass
        else:
            pass

# Ack Scan:
# A TCP packet with the ACK flag set and the port number to connect to is sent.
# If the server responds with the RST flag, the port is then unfiltered.
def ack(destination, ports, verbose):
    print "PORT\tSTATE"
    src_port = RandShort()

    ans, unans = sr(IP(dst=destination)/TCP(sport=src_port,flags="A",dport=ports), timeout=5, verbose=verbose)

    for s, r in ans:
        if s[TCP].dport == r[TCP].sport:
            print str(s[TCP].dport) + "/tcp\tunfiltered"
    
# Fin Scan:
# If there is no response from the server, then the port is open.
# If the server responds with an RST flag set in the TCP packet for the FIN scan request packet, 
# then the port is closed on the server.
def fin(destination, ports, verbose):
    print "PORT\tSTATE"
    for dst_port in ports:
        # Send request.
        resp = sr1(IP(dst = destination)/TCP(flags="F",dport=dst_port), timeout=2, verbose=verbose)

        if str(type(resp)) == "<type 'NoneType'>":
            print str(dst_port) + "/tcp\topen|filtered"
        elif resp.haslayer(TCP):
            if resp.getlayer(TCP.flags) == 0x14:
                print str(dst_port) + "/tcp\tclosed"
        elif resp.haslayer(ICMP):
            if int(resp.getlayer(ICMP)) == 3 and int(resp.getlayer(ICMP).code) in [1,2,3,9,10,13]:
                print str(dst_port) + "/tcp\tfiltered"
        else:
            print "CHECK"

# Xmas Scan:
# In Xmas scan, a TCP packet with the PSH, FIN and URG flags set along
# along with the port to connect to, is sent to the server. 
# If the port is open, there is no response received.
# If the server responds with a RST, the port is closed.
def xmas(destination, ports, verbose):
    print "PORT\tSTATE"
    for dst_port in ports:
        # Send request.
        resp = sr1(IP(dst=destination)/TCP(dport=dst_port,flags="FPU"),timeout=10, verbose=verbose)
    
        if (str(type(resp))=="<type 'NoneType'>"):
            print str(dst_port) + "/tcp\topen|filtered"
        elif(resp.haslayer(TCP)):
            if(resp.getlayer(TCP).flags == 0x14):
                print str(dst_port) + "/tcp\tclosed"

def ping(destination, ports):
    for dst_port in ports:
        ans, unans = sr(IP(dst=destination)/TCP(flags="S", dport=dst_port), timeout=10, verbose=verbose)
    
        for s, r in ans:
            print str(dst_port) + " is alive."

        if not ans:
            print str(dst_port) + " did not respond back."

def get_destination_ports(port_str):
    if not port_str:
        return list(range(0, 1024))
    elif '-' in port_str:
        a = port_str.split('-')
        return list(range(int(a[0]), int(a[1])+1))
    else:
        return [int(x) for x in port_str.split(',')]

if __name__=="__main__":
    arguments = docopt(__doc__, version = "3.2.2")

    ports = get_destination_ports(arguments['<port>'])
    destination = arguments['<destination>']
    technique = arguments['<scan>']
    verbose = arguments['--verbose']

    if verbose:
        verbose = 2
    else:
        verbose = 0
    
    locals()[technique](destination, ports, verbose)
