import telepot
import schedule, time
from bs4 import BeautifulSoup
import urllib

# Get's a random fact each day.
def get_message():
    f = urllib.urlopen("http://www.generatefacts.com")
    soup = BeautifulSoup(f)
    fact_string = soup.findAll('span', {'id': 'cpMain_MainFact_Home_lblRandomFact'})[0].string
    return "Good morning. Today's fact is: " + str(fact_string)

def job():
    # _id_ : Chat ID of the user you want to send messages
    # _API_TOKEN_ : API Token obtained from Telegram
    bot = telepot.Bot(_API_TOKEN_)
    messageToSend = get_message()
    bot.sendMessage(_id_, messageToSend)
    return

schedule.every().day.at("21:00").do(job)

while True:
    schedule.run_pending()
    time.sleep(60)
