# [![Build Status](https://img.shields.io/travis/kmein/menstruation-server.rs.svg?style=flat-square&logo=travis)](https://travis-ci.org/kmein/menstruation-server.rs) ![Size](https://img.shields.io/github/languages/code-size/kmein/menstruation-server.rs.svg?style=flat-square&logo=rust&logoColor=white) ![Size](https://img.shields.io/docker/automated/stoeppke/menstruation-server.rs.svg?style=flat-square)


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

* `/menu/<Mensa-Code>/<YYYY-MM-DD>` Gibt den Speiseplan der Mensa `<Mensa-Code>` am Tag `<YYYY-MM-DD>` zurück.

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


