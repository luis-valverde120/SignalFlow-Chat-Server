# SignalFlow: Visión General de la Arquitectura (Architecture Overview)

SignalFlow implementa un modelo de **Defensa en Profundidad (Defense-in-Depth)** estructurado en 5 capas concéntricas. Esta arquitectura asume que ninguna capa individual es infalible; si el anonimato de red se rompe, la criptografía protege el mensaje. Si la criptografía es extraída, el almacenamiento local destruye la evidencia. Si el dispositivo físico es comprometido activamente, el hardware colapsa el sistema.

## 1. El Modelo de 5 Capas (Defense-in-Depth)

La infraestructura lógica de SignalFlow se divide en las siguientes barreras defensivas:

- **[Capa 5 - Red y Enrutamiento](02_network_and_routing.md):** Mitiga el análisis de tráfico y el rastreo de IPs mediante una red 100% P2P sobre Tor, _Blind Dialing_ y asincronía local (_Store-and-Forward_).
- **[Capa 4 - Criptografía en Tránsito](03_cryptography.md):** Mitiga la intercepción de red mediante cifrado asíncrono _Double Ratchet_, garantizando _Perfect Forward Secrecy_ (PFS).
- **[Capa 3 - Seguridad Operativa (OpSec)](04_application_security.md):** Mitiga la vigilancia del sistema operativo host mediante evasión de capturas de pantalla (_Screen Shielding_) y teclados virtuales ofuscados.
- **[Capa 2 - Almacenamiento y Memoria](05_storage_and_memory.md):** Mitiga extracciones forenses en reposo mediante bases de datos efímeras cifradas (SQLCipher), purga de RAM (`zeroize`) y Negación Plausible.
- **[Capa 1 - Hardware Triggers](06_hardware_triggers.md):** Mitiga la coerción física y el _Live-Extraction_ mediante Pines de Coacción (_Duress PIN_) y Cordones de Desconexión de Hardware (_Hardware Kill Cord_).

```mermaid
flowchart TD
    %% Estilos de Capas (Degradado de colores fríos a cálidos/alerta)
    classDef layer5 fill:#1a365d,stroke:#63b3ed,stroke-width:2px,color:#fff
    classDef layer4 fill:#2b6cb0,stroke:#90cdf4,stroke-width:2px,color:#fff
    classDef layer3 fill:#2c5282,stroke:#a0aec0,stroke-width:2px,color:#fff
    classDef layer2 fill:#2a4365,stroke:#cbd5e0,stroke-width:2px,color:#fff
    classDef layer1 fill:#1e293b,stroke:#e53e3e,stroke-width:3px,color:#fff

    subgraph DefenseModel ["Arquitectura de Seguridad: Defensa en Profundidad"]
        direction TB

        L5["Capa 5: Red y Enrutamiento <br><br> (Tor-Only, Blind Dialing, Entropía Jitter)"]:::layer5
        L4["Capa 4: Criptografía en Tránsito <br><br> (Double Ratchet, Perfect Forward Secrecy)"]:::layer4
        L3["️ Capa 3: Seguridad Operativa (OpSec) <br><br> (Screen Shielding, Teclado Ofuscado)"]:::layer3
        L2["Capa 2: Almacenamiento y Memoria <br><br> (SQLCipher, Zeroize, Negación Plausible)"]:::layer2
        L1["Capa 1: Hardware Triggers (Core) <br><br> (Hardware Kill Cord, Duress PIN, Purga de RAM)"]:::layer1

        %% Flujo de penetración de amenazas
        L5 ==>|"Si el análisis de red penetra el anonimato..."| L4
        L4 ==>|"Si el tráfico es interceptado y guardado..."| L3
        L3 ==>|"Si el dispositivo está bajo vigilancia activa..."| L2
        L2 ==>|"Si el dispositivo es confiscado en reposo..."| L1

        %% El núcleo definitivo
        L1 -.->|"ÚLTIMA LÍNEA DE DEFENSA: <br> Destrucción del Sistema"| Collapse((Colapso <br> Seguro))
    end
```

---

## 2. Arquitectura de Sistemas y Topología de Código

Para lograr un aislamiento estricto y permitir que las defensas físicas funcionen 24/7, SignalFlow rechaza el modelo de aplicación tradicional de un solo hilo. El ecosistema está diseñado como un **Monorepo (Cargo Workspace)** que compila en dos procesos independientes:

### A. SignalFlow Sentinel (Demonio de Fondo)

El núcleo crítico del sistema. Es un binario sin interfaz gráfica (Headless) escrito en Rust puro que se ejecuta como un servicio de sistema (similar a un motor anticheat).

- **Responsabilidades:** Administra la red Tor, maneja la criptografía, vigila los puertos USB para el _Kill Cord_ y mantiene la base de datos cifrada abierta en memoria.

### B. SignalFlow UI (Frontend Tauri)

El cliente visual interactivo. Es una aplicación web encapsulada (TypeScript/React) que se comunica localmente con el _Sentinel Daemon_ a través de IPC (Inter-Process Communication) autenticado.

- **Responsabilidades:** Renderizar la interfaz de chat, recibir el input del usuario y ofrecer la experiencia de edición para la Negación Plausible. **No posee acceso directo a la memoria criptográfica.**

---

## 3. Diagrama de Arquitectura de Componentes

El siguiente diagrama ilustra el flujo de datos y la separación de responsabilidades a través de las librerías (`crates`) de nuestro Workspace en Rust:

```mermaid
flowchart TD
    %% Estilos de la Arquitectura
    classDef frontend fill:#1e1e1e,stroke:#4a90e2,stroke-width:2px,color:#fff
    classDef daemon fill:#2d3748,stroke:#e53e3e,stroke-width:2px,color:#fff
    classDef crate fill:#2b6cb0,stroke:#63b3ed,stroke-width:2px,color:#fff
    classDef storage fill:#4a5568,stroke:#ed8936,stroke-width:2px,color:#fff
    classDef external fill:#1a365d,stroke:#a0aec0,stroke-width:2px,color:#fff

    subgraph UserSpace ["Espacio de Usuario (Frontend)"]
        UI["Tauri UI App \n (React & TypeScript)"]:::frontend
    end

    subgraph Background ["Servicio 24/7 (Rust Daemon)"]
        Daemon{"SignalFlow Sentinel \n (Controlador de Estado)"}:::daemon

        HW_Trigger("(Hardware \n Kill Cord)"):::external
        HW_Trigger -- "Vigila USB/Estado" --> Daemon
    end

    subgraph Crates ["Librerías Core (Rust Workspace)"]
        C_Crypto["core_crypto \n (Double Ratchet + KDF)"]:::crate
        C_Net["network_tor \n (Arti + Blind Dialer)"]:::crate
        C_Storage["storage_vault \n (SQLCipher + Outbox)"]:::crate
    end

    subgraph Disk ["Almacenamiento Físico"]
        DB["(Bóveda Local \n SQLite Cifrado)"]:::storage
    end

    subgraph Net ["Internet"]
        Darknet("(Red Tor \n .onion)"):::external
    end

    %% Relaciones
    UI <== "IPC Local (Comandos)" ==> Daemon

    Daemon <--> C_Crypto
    Daemon <--> C_Net
    Daemon <--> C_Storage

    C_Storage == "PRAGMA secure_delete" ==> DB
    C_Net == "Tráfico Ofuscado" ==> Darknet
```
