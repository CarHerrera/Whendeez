---
Last Update: 2026-08-15
---

# Structure


## CT Side
![[Images/Mirage.png]]
- Alex watching apts
- Ari playing cat supporting mid
	- Can play passive just for info or try to get into ladder
- Aaron playing window/jungle and in general just rotating around 
- I will be playing conn/A site
	- Supporting mid early and then falling back to help A or mid again
- Jesus will be anchoring A
	- Mainly watching A ramp and palace
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "CT"
        - note["Used by"].containsAny("Aaron")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
#### Alex
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "CT"
        - note["Used by"].contains("Milan")
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

### Defaults

We should always be throwing one of [[All Instant Smokes]]
Should also be throwing one of [[Bottom Connector Smoke from T Spawn on Mirage]], [[Catwalk Smoke from T Spawn on Mirage]], [[Top Mid Smoke from T Spawn on Mirage]]

Alex will be going palace/ramp
Ari will be going mid mostly, same with aaron
Jesus will be going apts 
- Standard Default is this
- 1-3-1
	- ![[Mirage 1-3-1.png]]
	- Jesus does the usual
	- I will hold apts and lurk out mid through under if round goes well
	- Aaron Ari milan all go mid
- 1-2-2
	- ![[Mirage 1-2-2.png]]
	- Milan and I go into apts
	- Milan will go underpass
	- Ari and Aaron are gonna go mid
	- Jesus will lurk A either palace or ramp
- 4 mid Alex A

#### Some Duo Plays
### Execs
![[Mirage Execs.base]]

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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
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
#### Alex
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
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
        - Map.containsAny(link("../Mirage", "Mirage"), "Mirage")
        - and:
            - Side == "T"
        - note["Used by"].contains("Milan")
    filterBy:
      property: Lands
    groupBy:
      property: Lands
      direction: ASC
    image: note.image
    cardSize: 220
    imageFit: contain
```