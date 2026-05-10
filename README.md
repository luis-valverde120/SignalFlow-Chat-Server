# SignalFlow: Tor-Only Anti-Forensic Messaging Infrastructure

> **Nota del Desarrollador:** Este proyecto se encuentra actualmente en fase de diseño de arquitectura y modelado de amenazas (Threat Modeling). Este proyecto tiene varias limitaciones debido a que solo soy un estudiantes de ingenieria en ciencias de la computacion apasionado por la tecnologia y ciberseguridad, sin embargo me esfuerzo por aplicar buenas practicas de seguridad y seguir los estandares de la industria, y con este proyecto quiero aplicar mis habilidades en seguridad y desarrollo de software.

SignalFlow no es simplemente una aplicación de mensajería; es una infraestructura de comunicación diseñada bajo los principios estrictos de **Privacy by Design**, **Zero Trust** y **Minimización de Datos**. Su objetivo es proporcionar anonimato absoluto en la red y resistencia extrema contra extracciones forenses en vivo (Live-Extraction) o coerción física.

## Características Principales (Core Architecture Features)

SignalFlow se fundamenta en los principios de _Zero-Trust_, _Minimización de Datos_ y _Soberanía de Infraestructura_, diseñados para proteger comunicaciones sensibles en entornos hostiles:

- **Enrutamiento Zero-Trust (P2P Tor-Only):** Abandono total de servidores centrales y protocolos ruidosos (como WebRTC o UDP) que son susceptibles a la filtración de metadatos. Las conexiones se establecen de extremo a extremo exclusivamente a través de _Tor Hidden Services (.onion)_, mitigando ataques de correlación de red y garantizando la invisibilidad topológica.
- **Soberanía de Infraestructura (Self-Hosted Relays):** La aplicación no depende de nubes de terceros. Los mensajes pendientes se encolan localmente en el dispositivo de forma cifrada (Store-and-Forward asíncrono). Para escenarios de alta disponibilidad, se soporta la integración con _Nodos Relevo Personales_ (Buzones autohospedados en hardware del usuario, ej. Raspberry Pi), eliminando puntos únicos de fallo y jurisdicciones externas.
- **Minimización de Datos (Identidades Efímeras):** SignalFlow erradica el uso de Identificadores Personales (PII) como números de teléfono o correos electrónicos. La identidad de red se basa en pares de llaves criptográficas asimétricas (Ed25519) generadas on-device, las cuales pueden ser rotadas o "quemadas" operativamente en milisegundos para contener posibles brechas.
- **Postura Anti-Forense y Negación Plausible (Practical Deniability):** Diseñado para proteger a fuentes confidenciales e investigadores bajo coerción. La arquitectura de almacenamiento local permite la edición de la base de datos (SQLite/SQLCipher) por diseño. Esto invalida matemáticamente la legitimidad de cualquier captura de pantalla o volcado forense externo (_Live-Extraction_), frustrando los intentos de usar dispositivos confiscados como evidencia innegable.
- **Hardware Kill Cord & Duress PIN:** Mecanismos de defensa física que, bajo coerción o robo, despliegan una bóveda señuelo mientras purgan irreversiblemente las llaves reales en RAM (`zeroize`).

## Stack Tecnológico (Core en Rust)

- **Motor Criptográfico y Red:** Rust (Crates: `arti-client`, `x25519-dalek`, `zeroize`, `rusqlite`).
- **Frontend / UI:** Tauri (TypeScript / React) - Frontend ligero sin acceso directo a memoria criptográfica.
- **Mailbox Server:** Rust / Sled (Base de datos Key-Value incrustada para colas efímeras).
- **Red:** Tor Hidden Services (.onion).

## Índice de Documentación Arquitectónica (Architecture Deep Dive)

La ingeniería de SignalFlow está dividida en un modelo de defensa en profundidad (Defense-in-Depth) de 5 capas. Para entender las decisiones técnicas, mitigaciones y el modelado de amenazas, consulta la documentación detallada:

- **[Arquitectura](docs/00_architecture_overview.md):** Visión general del ecosistema, diagrama de componentes y el flujo de los datos a través del modelo híbrido.
- **[Modelo de Amenazas](docs/01_threat_model.md):** Modelado de Amenazas (Threat Modeling). Análisis de perfiles de adversarios, metodología STRIDE y los vectores de ataque explícitamente mitigados (y los que quedan fuera de alcance).
- **[Redes y Enrutamiento](docs/02_network_and_routing.md):** **Capa 5 (Red)**. Topología P2P Tor-Only, enrutamiento asíncrono, _Blind Dialing_, y defensas contra análisis estadístico de tráfico y _Sybil Attacks_.
- **[Criptografía](docs/03_cryptography.md):** **Capa 4 (Criptografía)**. Implementación del protocolo _Double Ratchet_, intercambio de llaves y garantías de _Perfect Forward Secrecy_ (PFS).
- **[Seguridad Operativa](docs/04_application_security.md):** **Capa 3 (Seguridad Operativa)**. Defensas activas contra _spyware_ a nivel de sistema operativo, _Screen Shielding_ y ofuscación de inputs.
- **[Almacenamiento y Memoria](docs/05_storage_and_memory.md):** **Capa 2 (Almacenamiento Local)**. SQLCipher, Bandeja de Salida Ciega (Blind Outbox), políticas destructivas de base de datos (`secure_delete`) y Negación Plausible Práctica.
- **[Hardware y Respuesta Física](docs/06_hardware_triggers.md):** **Capa 1 (Hardware & Respuesta Física)**. Mecanismos de emergencia _Anti-Live-Extraction_, integración del _Hardware Kill Cord_, _Duress PIN_ y protocolos de purga irreversible en RAM (`zeroize`).

## Descargo de Responsabilidad Legal y Ética

SignalFlow es un proyecto de investigación arquitectónica diseñado para explorar los límites de la criptografía aplicada y la mitigación de compromisos en dispositivos físicos. Está pensado para proteger a periodistas en zonas de conflicto, resguardar propiedad intelectual en cruces fronterizos hostiles, y proveer canales _Out-of-Band_ para equipos de Respuesta a Incidentes (IR).
