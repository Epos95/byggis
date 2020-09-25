# Build tool för Kattis

Byggis är ett verktyg för att lättare lösa [kattis](https://open.kattis.com) problem.

## Installation
Bygg från scratch via Github eller ladda ner via cargo. 
```bash
$ cargo install byggis
```
## Användning 

Skapa en ny mapp för ditt problem och ladda ner tests

## Användning 

Skapa en ny mapp för ditt problem och ladda ner tests från Kattis.
```bash
$ byggis new {namn på problem}
```

Skapa manuellt en main.* fil att skriva din kod i.
```bash
$ touch main.{extension}
```
Stöda språk:
* Python
* Rust

För att sedan testa din kod mot de tests på hemsidan för ditt problem.
```bash
$ byggis run
```

## TODO
- [ ] beskrivning av problem från kattis
- [ ] bättre hjälp meddelanden
- [ ] Fler språk

# Bidra
Just nu är de enda stödda språken Python och Rust. Om ditt föredragna språk inte stöds, bidra gärna genom att kontakta mig för mer information.

# Licens
[MIT](https://choosealicense.com/licenses/mit/)
