---
Last Update: 2026-08-15
---

# Structure


## CT Side

![[Dust2 Basic Setup.png]]
- Jesus B Anchor
- Aaron mid player
- I am cat/flex player
- Ari and Play are long (maybe ari main long)
- One of them can fall of and join for cat or supporting A 
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
        - Map.containsAny("Dust2")
        - Side.contains("CT")
        - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
        - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map == "Dust2"
        - and:
            - Side == "CT"
        - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
```
#### Platypus
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
```


## T Side

![[images/Pasted image 20251211094429.png]]
### Defaults

### Execs

![[Dust2/Execs.base|Execs]]
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
        - Map.containsAny("Dust2")
        - Side.contains("T")
        - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - and:
                - Side == "T"
        - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("All")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("Ari")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("Aaron")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - and:
            - Side == "T"
            - note["Used by"].contains("Carlos")
        - Map == "Dust2"
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("Carlos")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.contains("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("Jesus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
```
#### Platypus
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
            - note["Used by"].contains("Platypus")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
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
        - Map.containsAny("Dust2")
        - and:
            - Side == "T"
        - note["Used by"].contains("Platypus")
    groupBy:
      property: Lands
      direction: ASC
    filterBy:
      property: Lands
    image: note.image
    cardSize: 220
    imageFit: contain

```

