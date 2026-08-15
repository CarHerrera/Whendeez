---
Last Update: 2026-08-15
---

# Structure


## CT Side
- Jesus will be the A anchor
- Ari and Milan will be the B players 
- Aaron will be awping and he can go anywhere. Can start archest, banana or apts
- I will be the other A player holding the CT/library side of A 

### Setups


- Basic CT Setup
	- ![[Basic Setup.png| Basic CT Setup|671]]
- Banana Peak 
	- ![[Banana Peak Setup.png]]
- Early B Stack
	- ![[Early B Stack.png]]
#### Some Duo Plays
- Ari and Milan 
	- ![[First Orange Coffins Setup]]
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
        - Map.containsAny("Inferno")
        - Side == "CT"
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
        - and:
            - Side == "CT"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```
### Some special plays


## T Side

### Defaults

- Our typical default is gonna be a 2-1-2
	- Ari and I will be going Banana
	- Milan will be going in Apts
		- Can go through T apts or bed room
		- Be sure to call what you hear and also ask for utility to help entry out A
	- Jesus will be supporting apts and also establishing mid presence
	- ![[Basic default.png]]
- 3-1-1 (3 A)
	- Ari would be holding Banana
	- I would be pushing up mid and throwing more supportive util 
	- Aaron would be support Ari with util or gunfire
	- ![[Images/Inferno.png]]
- 1-1-3 
	- Aaron would be joining us into Banana (Me and Ari)
	- Milan Would be playing mid passively holding for a push
	- Jesus would be holding mid and maybe throw util for banana 
### Execs
![[Inferno Execs.base]]
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
        - Map.containsAny("Inferno")
        - Side == "T"
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
        - Map.containsAny("Inferno")
        - and:
            - Usage == ["Default"]
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
        - Map.containsAny("Inferno")
        - and:
            - Usage == ["Default"]
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
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
        - Map.containsAny("Inferno")
        - and:
            - Side == "T"
            - note["Used by"].contains("Milan")
    image: note.image
    cardSize: 220

```