# Worstbird

The repo that hosts worstbird.eu.

## About:

Die Webseite stellt automatisch jeden Monat 5 neue Vögel zur schau, für die Abgestimmt werden kann. Der Vogel mit den meisten "Votes" ist Worstbird des Monats. Bei gleicher Stimmenanzahl gibt es mehrere Worstbirds im Monat. Alle Worstbirds aller Monate des Jahres werden im Januar des darauffolgenden Jahres automatisch in einen Topf geworfen und man kann für den Worstbird des Jahres "voten".

Die Bilder und Beschreibungen kommen von [ebird.org](https://ebird.org)

## Projekt

Das Projekt is öffentlich und jeder kann daran teilhaben etc. [Github](https://github.com/Heusini/worstbird/)
Es gibt drei Sub-Projekte:
### worstbird_http:
Das Webserver Backend, dass die Vögel aus der Datenbank lädt und diw einezelnen Webseiten generiert und bereitstellt.
### worstbird_fetcher:
Der Dienst der im hintergrund monatlich neue zufällige Vögel raussucht und in die Datenbank einpflegt. (Hier findet die automatisierung statt)
### worstbird_twitter: 
Der Dienst der in zukunft (noch nicht implementiert) jeden Monat den Worstbird des Monats ankündigen soll.
### worstbird_db:
Helper für die Datenbank abfragen.

## Warum das Projekt?

Man weiß es nicht.
