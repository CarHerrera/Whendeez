---
Last Update: 2026-08-15
---

# Structure


## CT Side


### Setups
Basic Roles
- Aaron and Ari will be playing mid.
- Aaron will be playing highway 
- Ari will play Z
- I will be the B rotator
- Plat is the B Anchor
- Jesus is the A Anchor
![[Cache-1.png]]

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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
        - and:
            - Side == "CT"
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
        - Map.containsAny("Cache")
        - and:
            - Side == "CT"
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
        - Map.containsAny("Cache")
        - and:
            - Side == "CT"
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
        - Map.containsAny("Cache")
        - and:
            - Side == "CT"
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


## T Side


- Jesus Lurk hold A
- I hold B main
- Other players mid taking control or controllng the extremeties
### Defaults

- Basic Default (1-3-1)
- ![[Cache 1-3-1-1.png]]
	- Jesus 
		- will go A main and hold to make sure they aren't going aggro
		- Can double back and help out mid
	- Aaron 
		- will probably get boosted but if not would be holding squeaky getting ready to throw util
	- Ari
		- Getting boosted or boosting.
		- Can then go mid to help get control
	- MIlan getting boosted or boosting.
		- Same as Ari
	- I will be holding sunroom trying not to get picked 
- 0-3-2 
	- 
	- This will probably be for a B split
	- Aaron will join me on the B lurk
	- Jesus Ari and MIlan will be trying to do a late mid boost into mid control 
- 4 - 1
	- 4 Towards A. Someone can Join jesus in A main
	- I will be making sure they don't push B main aggro 
Please when boosted don't just run immediately wait for a synchronized peak 
#### Some Duo Plays
### Execs
![[Cache/Execs.base|Execs]]
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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
        - Map.containsAny("Cache")
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

