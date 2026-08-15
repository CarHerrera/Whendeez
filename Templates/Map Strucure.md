---
Last Update: 2026-08-15
---

# Structure


## CT Side


### Setups
- Basic CT Setup

#### Some Duo Plays

### Util
#### All of Us
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flash", "Flashbang")
        - Map.containsAny("{{title}}")
        - Side.contains("CT")
        - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```

##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
        - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```

#### Ari
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```
#### Aaron
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```
#### Carlos
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```
#### Jesus

##### Flashes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```
#### Milan
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```

##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```


## T Side

### Defaults


#### Some Duo Plays
### Execs

### Util
This is util I expect Y'all to know
#### All of Us
##### Flashes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flash", "Flashbang")
        - Map.containsAny("{{title}}")
        - Side.contains("T")
        - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```
##### Smokes
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - and:
                - Side == "T"
        - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```
##### Mollies
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```

##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("All")
    image: note.image
    cardSize: 220

```
#### Ari
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```

##### Smokes

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```

##### Mollies

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("Ari")
    image: note.image
    cardSize: 220

```
#### Aaron
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```

##### Smokes

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```

##### Mollies

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("Aaron")
    image: note.image
    cardSize: 220

```
#### Carlos
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```

##### Smokes

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```

##### Mollies

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("Carlos")
    image: note.image
    cardSize: 220

```
#### Jesus

##### Flashes

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```

##### Smokes

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```

##### Mollies

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```
##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("Jesus")
    image: note.image
    cardSize: 220

```
#### Milan
##### Flashbang
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("Flashbang", "Flash")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```

##### Smokes

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Smoke")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```

##### Mollies

```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.contains("Molotov")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```

##### HE
```base
filters:
  and:
    - file.hasProperty("Nade")
    - "!Nade.isEmpty()"
views:
  - type: cards
    name: Table
    filters:
      and:
        - Nade.containsAny("HE", "Grenade")
        - Map.containsAny("{{title}}")
        - and:
            - Side == "T"
        - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```