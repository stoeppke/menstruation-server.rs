# menstruation-server.rs [![Build Status](https://travis-ci.org/kmein/menstruation-server.rs.svg?branch=master)](https://travis-ci.org/kmein/menstruation-server.rs)

## API-Beschreibung

* `/codes` Listet alle Codes der Mensen des Studentenwerks Berlin auf.

```json
[
  {
    "items": [
      {
        "address": "<Adresse>",
        "name": "<Name>",
        "code": "<Code>"
      }
    ],
    "name": "<Uni-Name>"
  }
]
```

* `/menu/<Mensa-Code>/<YYYY-MM-DD>` Gibt den Speiseplan der Mensa `<Mensa-Code>` am Tag `<YYYY-MM-DD>` zurück. Dabei ist das Datum optional, und es wird der aktuelle Speiseplan zurückgegeben.

```json
[
  {
    "items": [
      {
        "allergens": ["<Allergene>"],
        "color": "<green|yellow|red>",
        "name": "<Essensname>",
        "price": {
          "student": <Studentenpreis>,
          "employee": <Angestelltenpreis>,
          "guest": <Gästepreis>
        },
        "tags": ["<Kennzeichen>"]
      }
    ],
    "name": "<Gruppenname>"
  }
]
```


