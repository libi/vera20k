# VERA Mod Engine

[English](README.md) · [简体中文](README.zh-CN.md) · **Deutsch**

VERA Mod Engine ist ein in Rust geschriebener Fork von VERA20K für Mods und Missionen zu *Alarmstufe Rot 2* (Red Alert 2, RA2). YuriPlanet entwickelt den Fork eigenständig weiter. Mod-Autoren sollen einzelne Spielmechaniken per INI ändern und dabei den Technologiebaum, die Missionen und die übrigen Regeln des gewählten Spiels beibehalten können. So könnte eine RA2-Mod einem einzelnen Sprengkopf einen Verlangsamungseffekt nach dem Vorbild von Ares geben.

**Entwicklungsstand.** Das Repository enthält die VERA20K-Engine, die Entwurfsdokumentation und drei maschinenlesbare Referenzlisten. Sie erfassen die INI-Dateien der Originalspiele, die von den Originalprogrammen gelesenen INI-Schlüssel und die im Ares-Handbuch dokumentierten Einträge. Die folgenden Kompatibilitätsprofile und die neue INI-Syntax befinden sich im Entwurfsstadium.

## Kompatibilitätsziele

Jedes Profil legt fest, welche Dateien die Engine lädt, wie sie INI-Schlüssel auswertet und welche Ereignis- und Aktionsnummern in Missionen gelten. Für Bewegung, Waffenfeuer, Schadensberechnung, Attach Effects (AE) und Kartentrigger ist eine gemeinsame Simulationslogik vorgesehen.

| Profil | Regeln und Missionen | Referenz für den Abgleich |
|---|---|---|
| Alarmstufe Rot 2 | `rules.ini`, `art.ini`, `ai.ini` und RA2-Karten | Originalversion von RA2 |
| Yuris Rache | `rulesmd.ini`, `artmd.ini`, `aimd.ini` und YR-Karten, einschließlich der Regeländerungen durch Spielmodus und Karte | Kampagnen und Gefechte der Originalversion von YR |
| Ares | YR-Definitionen sowie INI-Schlüssel, Triggernummern und Änderungen am Standardverhalten der gewählten Ares-Version | YR 1.001 mit Ares 3.0 bzw. 3.0p1, jeweils als eigene Referenz |

Im vorgeschlagenen INI-Format verbindet ein Trigger ein Ereignis mit einem Filter und einem Effekt. Ein elektrisches Feld kann beispielsweise Infanteristen und Geschossen vorübergehend das Flag `Tesla` zuweisen. Solange sie das Flag tragen, kann ein zweiter Trigger beide über denselben Filter auswählen.

Für die erste Ausbaustufe sind handgeschriebene INI-Dateien, INIWeaver-Vorlagen und ein Nachschlagewerk für die einzelnen Felder vorgesehen. Die Unterstützung für eigene WebAssembly-Module folgt laut Entwurf später.

## Geschossflug mit fester Flugzeit

Im ersten Meilenstein werden auf derselben YR-Karte die `105mm`-Standardkanone des Grizzly-Panzers und eine zweite Kanone verglichen. Die zweite Kanone übernimmt den ursprünglichen Sprengkopf. Für ihr Geschoss sieht der Entwurf folgende Bewegungsdefinition vor.

```ini
[Motion demo.HighArc]
Goal=SnapshotAtStart
Path=Line
Pace=FixedTicks(60)
Height=PeakAboveHigherEnd(192lepton)
Facing=AlongPath
```

Bei freier Flugbahn soll das Geschoss nach 60 Simulationsticks die beim Abschuss erfasste Zielposition erreichen. Der Wert `192lepton` bestimmt die Höhe des Scheitelpunkts über dem höheren der beiden Endpunkte. Die Flugzeit bleibt beim Ändern dieses Werts gleich. Für die Abnahme muss ein Geschoss, das vorzeitig auf eine Brücke trifft, genau einmal detonieren. Nach dem Laden eines während des Flugs gespeicherten Spielstands muss der Einschlag im selben Tick erfolgen wie bei ununterbrochenem Spiel. [Vollständiges Beispiel mit Waffe und Geschoss](docs/zh-CN/mod-engine-product-guide.md#配置示例--分别设定飞行时间与弧高)

## Dokumentation

Für die aktuelle Engine gibt es englische Anleitungen zum Einrichten von Mods, zum Kompilieren und zum Testen. [Modding-Handbuch](docs/modding.md) · [Entwicklerwerkzeuge](tools/README.md)

Die Spezifikationen liegen in vereinfachtem Chinesisch vor.

- [Produkthandbuch mit Beispielen für Mod-Autoren](docs/zh-CN/mod-engine-product-guide.md)
- [INI-Kompatibilität und Quellenverzeichnisse](docs/zh-CN/ini-preset-wrapping.md)
- [Spezifikation der Bewegungen, Regeln und Effekte](docs/zh-CN/composable-gameplay-design.md)
- [Entwicklungsplan und Abnahmekriterien](docs/zh-CN/composable-gameplay-roadmap.md)
- [Vorschläge, Antworten der Beteiligten und offene Fragen zum RFC #4653 im Ursprungsprojekt](docs/zh-CN/rfc-4653-feedback.md)

## Kompilieren und starten

Der aktuelle Desktop-Client liest die Spieldaten aus einer lokalen Installation von *Yuris Rache*. Zum Kompilieren werden Git und die über rustup installierte stabile Rust-Version benötigt. Unter Windows kommen die Visual Studio C++ Build Tools hinzu, unter macOS die Xcode Command Line Tools. Unter Debian oder Ubuntu werden `build-essential`, `pkg-config`, `libasound2-dev` und `libxkbcommon-x11-0` benötigt. [Rust-Toolchain](rust-toolchain.toml)

```sh
git clone https://github.com/YuriPlanet/vera-mod-engine.git
cd vera-mod-engine
```

Kopiere `config.toml.example` nach `config.toml` und trage unter `[paths]` bei `ra2_dir` das Installationsverzeichnis des Spiels ein. Starte den Client anschließend im Stammverzeichnis des Repositorys.

```sh
cargo run --locked --profile fast --bin vera20k
```

## Mitwirken

Gib bei Kompatibilitätsproblemen die Versionen des Spiels und der Erweiterung an. Füge den kleinsten INI-Abschnitt oder FinalAlert-2-Trigger bei, mit dem sich das Problem nachstellen lässt, und beschreibe das Verhalten im Original und im Fork. Benenne bei einer Änderung an der Engine das betroffene Spielverhalten und das Szenario, mit dem es sich überprüfen lässt. [Issues des Projekts](https://github.com/YuriPlanet/vera-mod-engine/issues)

## Lizenz und Urheber

Copyright © 2026 Halvor Nikolai Rusten und die Mitwirkenden an VERA20K. Der Quellcode der Engine steht unter der GPLv3 mit den zusätzlichen Bestimmungen des Projekts. Command & Conquer und Red Alert sind Marken von Electronic Arts Inc. [Ursprungsprojekt](https://github.com/YuriPlanet/vera20k) · [GPLv3](LICENSE-GPL) · [Rechtliche Hinweise und zusätzliche Bestimmungen](LEGAL.md#additional-terms)

This product includes software developed by the Ares Contributors.
This product includes software developed by the Phobos Developers. [Ares-Lizenz](LICENSES/Ares.txt) · [Mitwirkende an Phobos](docs/phobos.md#original-project-and-authors)
